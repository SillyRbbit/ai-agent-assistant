//! C1: deterministic examples only. This module has no I/O or runtime dependencies.
use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use thiserror::Error;

const SAFE: u64 = 9_007_199_254_740_991;
const LIMIT: usize = 128;
pub const DISCLOSURE: &str = "Mock only — Cortexa-shaped examples, not your activity.";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Absence {
    Missing,
    Unavailable,
    Unverified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum Count {
    Value(u64),
    Absent(Absence),
}

const ZERO: Count = Count::Value(0);
const MISSING: Count = Count::Absent(Absence::Missing);
const UNAVAILABLE: Count = Count::Absent(Absence::Unavailable);
const UNVERIFIED: Count = Count::Absent(Absence::Unverified);

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(untagged)]
enum Ratio {
    Value(f64),
    Absent(Absence),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Category {
    Simulation,
    OpenaiApi,
    AnthropicApi,
    LmStudio,
    Ollama,
    Codex,
}

const CATEGORIES: [Category; 6] = [
    Category::Simulation,
    Category::OpenaiApi,
    Category::AnthropicApi,
    Category::LmStudio,
    Category::Ollama,
    Category::Codex,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Representation {
    Cumulative,
    Delta,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Completed,
    Cancelled,
    Failed,
}

/// Identity is internal and synthetic. No account, turn or parent aggregates enter this type.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Leaf {
    category: Category,
    id: u8,
    task: u8,
    day: u8,
    sequence: u8,
    representation: Representation,
    status: Status,
    tokens: [Count; 4],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Daily {
    date: String,
    tokens: [Count; 4],
    total: Count,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct View {
    category: Category,
    days: u8,
    example: &'static str,
    start: String,
    end: &'static str,
    entries: usize,
    completed_tasks: usize,
    tokens: [Count; 4],
    total: Count,
    cache_percent: Ratio,
    completed_task_mean: Ratio,
    daily: Vec<Daily>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageEfficiencyProjection {
    schema_version: u8,
    provenance: &'static str,
    disclosure: &'static str,
    views: Vec<View>,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UsageEfficiencyError {
    #[error("Usage & Efficiency examples unavailable.")]
    Unavailable,
}
type Result<T> = std::result::Result<T, UsageEfficiencyError>;

fn need(valid: bool) -> Result<()> {
    if valid {
        Ok(())
    } else {
        Err(UsageEfficiencyError::Unavailable)
    }
}
fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .filter(|v| *v <= SAFE)
        .ok_or(UsageEfficiencyError::Unavailable)
}
fn sum(values: impl IntoIterator<Item = Count>) -> Result<Count> {
    let mut known = 0;
    let mut missing = false;
    let mut unavailable = false;
    let mut unverified = false;
    for value in values {
        match value {
            Count::Value(n) => {
                need(n <= SAFE)?;
                known = add(known, n)?;
            }
            Count::Absent(Absence::Missing) => missing = true,
            Count::Absent(Absence::Unavailable) => unavailable = true,
            Count::Absent(Absence::Unverified) => unverified = true,
        }
    }
    // S1 precedence and repaired additive zero identity, not zero imputation.
    Ok(if unverified {
        UNVERIFIED
    } else if missing || (unavailable && known > 0) {
        MISSING
    } else if unavailable {
        UNAVAILABLE
    } else {
        Count::Value(known)
    })
}
fn subset(part: Count, whole: Count) -> Result<()> {
    if let (Count::Value(p), Count::Value(w)) = (part, whole) {
        need(p <= w)?;
    }
    Ok(())
}
fn validate_tokens(tokens: [Count; 4]) -> Result<()> {
    for token in tokens {
        if let Count::Value(n) = token {
            need(n <= SAFE)?;
        }
    }
    subset(tokens[1], tokens[0])?;
    subset(tokens[3], tokens[2])
}
fn totals(rows: &[&Leaf]) -> Result<[Count; 4]> {
    let mut result = [ZERO; 4];
    for (i, metric) in result.iter_mut().enumerate() {
        *metric = sum(rows.iter().map(|row| row.tokens[i]))?;
    }
    validate_tokens(result)?;
    Ok(result)
}
fn canonical(input: &[Leaf]) -> Result<Vec<Leaf>> {
    need(input.len() <= LIMIT)?;
    let mut groups: BTreeMap<(Category, u8), BTreeMap<u8, &Leaf>> = BTreeMap::new();
    let mut tasks = BTreeMap::new();
    for row in input {
        need(row.day > 0 && row.day <= 30 && row.sequence < 128 && row.id < 128 && row.task < 128)?;
        validate_tokens(row.tokens)?;
        let task = (row.category, row.task);
        if let Some(status) = tasks.insert(task, row.status) {
            need(status == row.status)?;
        }
        let revisions = groups.entry((row.category, row.id)).or_default();
        if let Some(prior) = revisions.insert(row.sequence, row) {
            need(prior == row)?;
        }
    }
    let mut result = Vec::new();
    for revisions in groups.values() {
        let first = revisions
            .first_key_value()
            .ok_or(UsageEfficiencyError::Unavailable)?
            .1;
        for row in revisions.values() {
            need(
                row.task == first.task
                    && row.status == first.status
                    && row.day == first.day
                    && row.representation == first.representation,
            )?;
        }
        let last = revisions
            .last_key_value()
            .ok_or(UsageEfficiencyError::Unavailable)?
            .1;
        let mut output = (*last).clone();
        match first.representation {
            Representation::Cumulative => {
                let mut known = [None; 4];
                for row in revisions.values() {
                    for (index, metric) in row.tokens.iter().enumerate() {
                        if let Count::Value(n) = metric {
                            if let Some(previous) = known[index] {
                                need(*n >= previous)?;
                            }
                            known[index] = Some(*n);
                        }
                    }
                }
                // Keep final absence; earlier known counts are not backfilled.
            }
            Representation::Delta => {
                for (expected, actual) in revisions.keys().enumerate() {
                    need(usize::from(*actual) == expected)?;
                }
                let refs: Vec<&Leaf> = revisions.values().copied().collect();
                output.tokens = totals(&refs)?;
            }
        }
        result.push(output);
    }
    Ok(result)
}
fn ratio(part: Count, whole: Count, factor: f64) -> Ratio {
    match (part, whole) {
        (Count::Value(p), Count::Value(w)) if w > 0 => Ratio::Value(p as f64 / w as f64 * factor),
        (Count::Value(_), Count::Value(_)) => Ratio::Absent(Absence::Unavailable),
        (Count::Absent(a), _) => Ratio::Absent(a),
        _ => Ratio::Absent(Absence::Missing),
    }
}
fn project(all: &[Leaf], category: Category, days: u8, empty: bool) -> Result<View> {
    need(days == 7 || days == 30)?;
    let start = 31 - days;
    let selected: Vec<&Leaf> = all
        .iter()
        .filter(|r| !empty && r.category == category && r.day >= start)
        .collect();
    let tokens = totals(&selected)?;
    let total = sum([tokens[0], tokens[2]])?;
    let tasks: BTreeSet<u8> = selected
        .iter()
        .filter(|r| r.status == Status::Completed)
        .map(|r| r.task)
        .collect();
    let mut complete = true;
    let mut task_counts = Vec::new();
    for task in &tasks {
        let rows: Vec<&Leaf> = all
            .iter()
            .filter(|r| r.category == category && r.task == *task)
            .collect();
        complete &= rows.iter().all(|r| r.day >= start);
        let counts = totals(&rows)?;
        let count = sum([counts[0], counts[2]])?;
        complete &= matches!(count, Count::Value(_));
        task_counts.push(count);
    }
    let completed_task_mean = if !complete {
        Ratio::Absent(Absence::Missing)
    } else {
        ratio(sum(task_counts)?, Count::Value(tasks.len() as u64), 1.0)
    };
    let mut daily = Vec::new();
    // Empty is an explicit complete zero-activity illustration, with no daily records.
    if !empty {
        for day in start..=30 {
            let rows: Vec<&Leaf> = selected.iter().filter(|r| r.day == day).copied().collect();
            let day_tokens = totals(&rows)?;
            daily.push(Daily {
                date: format!("2026-09-{day:02}"),
                tokens: day_tokens,
                total: sum([day_tokens[0], day_tokens[2]])?,
            });
        }
        for (index, metric) in tokens.iter().enumerate() {
            need(sum(daily.iter().map(|d| d.tokens[index]))? == *metric)?;
        }
        need(sum(daily.iter().map(|d| d.total))? == total)?;
    }
    Ok(View {
        category,
        days,
        example: if empty { "empty" } else { "activity" },
        start: format!("2026-09-{start:02}"),
        end: "2026-10-01",
        entries: selected.len(),
        completed_tasks: tasks.len(),
        tokens,
        total,
        cache_percent: ratio(tokens[1], tokens[0], 100.0),
        completed_task_mean,
        daily,
    })
}
fn fixtures() -> Vec<Leaf> {
    let mut rows = Vec::new();
    for category in CATEGORIES {
        let (cache, reasoning) = match category {
            Category::AnthropicApi => (Count::Value(0), UNAVAILABLE),
            Category::LmStudio => (MISSING, Count::Value(10)),
            Category::Ollama => (Count::Value(0), UNVERIFIED),
            Category::Codex => (MISSING, Count::Value(10)),
            Category::OpenaiApi => (Count::Value(0), Count::Value(10)),
            Category::Simulation => (Count::Value(100), Count::Value(10)),
        };
        let row = |id, task, day, input, output, status| Leaf {
            category,
            id,
            task,
            day,
            sequence: 0,
            representation: Representation::Cumulative,
            status,
            tokens: [Count::Value(input), cache, Count::Value(output), reasoning],
        };
        rows.push(row(1, 1, 4, 800, 150, Status::Completed));
        rows.push(row(2, 2, 18, 250, 130, Status::Completed));
        rows.push(row(3, 2, 24, 500, 100, Status::Completed));
        let first = row(4, 3, 27, 200, 25, Status::Failed);
        rows.push(first.clone());
        let mut last = first;
        last.sequence = 1;
        last.tokens[0] = Count::Value(400);
        last.tokens[2] = Count::Value(50);
        rows.push(last.clone());
        rows.push(last); // identical retransmission counts once
        let mut delta = row(5, 4, 30, 300, 50, Status::Cancelled);
        delta.representation = Representation::Delta;
        rows.push(delta.clone());
        delta.sequence = 1;
        rows.push(delta);
    }
    rows
}
pub fn current_usage_efficiency_demo_projection() -> Result<UsageEfficiencyProjection> {
    let rows = canonical(&fixtures())?;
    let mut views = Vec::new();
    for category in CATEGORIES {
        for days in [7, 30] {
            for empty in [false, true] {
                views.push(project(&rows, category, days, empty)?);
            }
        }
    }
    need(views.len() <= 24)?;
    let result = UsageEfficiencyProjection {
        schema_version: 1,
        provenance: "synthetic",
        disclosure: DISCLOSURE,
        views,
    };
    let bytes = serde_json::to_vec(&result).map_err(|_| UsageEfficiencyError::Unavailable)?;
    need(bytes.len() <= 64 * 1024)?;
    Ok(result)
}
#[tauri::command]
pub(crate) fn get_usage_efficiency_demo_projection() -> Result<UsageEfficiencyProjection> {
    current_usage_efficiency_demo_projection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_is_bounded_deterministic_and_source_separated() -> Result<()> {
        let a = get_usage_efficiency_demo_projection()?;
        assert_eq!(a, current_usage_efficiency_demo_projection()?);
        assert_eq!(a.views.len(), 24);
        assert!(fixtures().len() <= 128);
        for category in CATEGORIES {
            for days in [7, 30] {
                let view = project(&canonical(&fixtures())?, category, days, false)?;
                assert_eq!(
                    view.total,
                    Count::Value(if days == 7 { 1750 } else { 3080 })
                );
                assert_eq!(view.entries, if days == 7 { 3 } else { 5 });
                assert_eq!(view.completed_tasks, if days == 7 { 1 } else { 2 });
                assert_eq!(view.daily.len(), usize::from(days));
                assert_eq!(
                    view.completed_task_mean,
                    if days == 7 {
                        Ratio::Absent(Absence::Missing)
                    } else {
                        Ratio::Value(965.0)
                    }
                );
            }
        }
        Ok(())
    }
    #[test]
    fn repaired_unavailable_zero_padding_reconciles() -> Result<()> {
        for days in [7, 30] {
            let view = project(
                &canonical(&fixtures())?,
                Category::AnthropicApi,
                days,
                false,
            )?;
            assert_eq!(view.tokens[3], UNAVAILABLE);
            assert_eq!(sum(view.daily.iter().map(|d| d.tokens[3]))?, UNAVAILABLE);
        }
        assert_eq!(sum([ZERO, UNAVAILABLE])?, UNAVAILABLE);
        assert_eq!(sum([Count::Value(1), UNAVAILABLE])?, MISSING);
        assert_eq!(sum([MISSING, UNVERIFIED])?, UNVERIFIED);
        for a in [ZERO, Count::Value(3), MISSING, UNAVAILABLE, UNVERIFIED] {
            for b in [ZERO, Count::Value(3), MISSING, UNAVAILABLE, UNVERIFIED] {
                for c in [ZERO, Count::Value(3), MISSING, UNAVAILABLE, UNVERIFIED] {
                    assert_eq!(sum([sum([a, b])?, c])?, sum([a, sum([b, c])?])?);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn repaired_last_known_decrease_rejects_across_each_absence() -> Result<()> {
        for index in 0..4 {
            for absent in [MISSING, UNAVAILABLE, UNVERIFIED] {
                let mut first = fixtures()[0].clone();
                first.tokens = [Count::Value(10); 4];
                let mut missing = first.clone();
                missing.sequence = 1;
                missing.tokens[index] = absent;
                let mut last = first.clone();
                last.sequence = 2;
                last.tokens[index] = Count::Value(9);
                // Ensure the subset check doesn't mask the across-absence comparison.
                if index == 0 {
                    last.tokens[1] = Count::Value(9);
                }
                if index == 2 {
                    last.tokens[3] = Count::Value(9);
                }
                assert!(canonical(&[first.clone(), missing.clone(), last]).is_err());
                let result = canonical(&[missing.clone(), first, missing])?;
                assert_eq!(result[0].tokens[index], absent);
            }
        }
        Ok(())
    }
    #[test]
    fn duplicate_delta_gap_mixture_identity_and_bounds_reject() {
        let base = fixtures()[0].clone();
        let mut bad = base.clone();
        bad.tokens[0] = Count::Value(999);
        assert!(canonical(&[base.clone(), bad]).is_err());
        let mut bad = base.clone();
        bad.representation = Representation::Delta;
        bad.sequence = 1;
        assert!(canonical(&[bad.clone()]).is_err());
        bad.sequence = 1;
        assert!(canonical(&[base.clone(), bad]).is_err());
        let mut bad = base.clone();
        bad.day = 0;
        assert!(canonical(&[bad]).is_err());
        let mut bad = base.clone();
        bad.tokens[1] = Count::Value(801);
        assert!(canonical(&[bad]).is_err());
        let mut bad = base.clone();
        bad.tokens[0] = Count::Value(SAFE + 1);
        assert!(canonical(&[bad]).is_err());
        assert!(canonical(&vec![base; 129]).is_err());
        assert!(sum([Count::Value(SAFE), Count::Value(1)]).is_err());
    }
    #[test]
    fn empty_zero_denominators_and_incomplete_tasks_are_explicit() -> Result<()> {
        let all = canonical(&fixtures())?;
        let empty = project(&all, Category::Codex, 7, true)?;
        assert_eq!(empty.tokens, [ZERO; 4]);
        assert_eq!(empty.total, ZERO);
        assert_eq!(
            empty.completed_task_mean,
            Ratio::Absent(Absence::Unavailable)
        );
        assert_eq!(empty.cache_percent, Ratio::Absent(Absence::Unavailable));
        assert!(empty.daily.is_empty());
        let mut rows = all;
        let row = rows
            .iter_mut()
            .find(|r| r.category == Category::Simulation && r.id == 3)
            .ok_or(UsageEfficiencyError::Unavailable)?;
        row.tokens[0] = MISSING;
        assert_eq!(
            project(&rows, Category::Simulation, 30, false)?.completed_task_mean,
            Ratio::Absent(Absence::Missing)
        );
        Ok(())
    }
}

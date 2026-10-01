// Shared centered-row geometry; presentation only, never a workflow contract.
export function centeredRowX(
  count: number,
  column: number,
  width: number,
  gap: number,
  available: number,
): number {
  return (available - (count * width + Math.max(0, count - 1) * gap)) / 2 + column * (width + gap);
}

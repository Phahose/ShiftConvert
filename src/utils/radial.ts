
export function getFormatAtPosition(
  mouseX: number,
  mouseY: number,
  formats: string[],
  centerX: number,
  centerY: number,
  radius = 130,
  buttonRadius = 32.5
): string | null {
  for (let index = 0; index < formats.length; index++) {
    const angle = (index * 2 * Math.PI) / formats.length;

    const buttonX = centerX + radius * Math.sin(angle);
    const buttonY = centerY - radius * Math.cos(angle);

    const dx = mouseX - buttonX;
    const dy = mouseY - buttonY;

    // Check if the mouse position is within the button's radius
    if (dx * dx + dy * dy <= buttonRadius * buttonRadius) {
      return formats[index];
    }
  }

  return null;
}

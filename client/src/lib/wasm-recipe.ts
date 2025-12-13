export async function generateRecipeSvg(recipeText: string): Promise<string> {
  const lines = recipeText.split('\n');
  const titleMatch = lines[0]?.match(/^#\s*(.+)/) || lines[0]?.match(/^(.+)/);
  const title = titleMatch?.[1] || 'Recipe';
  
  const contentLines = lines.slice(1).filter(line => line.trim());
  
  const lineHeight = 24;
  const padding = 40;
  const titleHeight = 60;
  const contentStartY = padding + titleHeight + 20;
  const svgHeight = contentStartY + (contentLines.length * lineHeight) + padding;
  const svgWidth = 600;

  const escapeHtml = (text: string) => {
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  };

  const contentSvg = contentLines.map((line, index) => {
    const y = contentStartY + (index * lineHeight);
    const escapedLine = escapeHtml(line);
    
    if (line.startsWith('## ')) {
      return `<text x="${padding}" y="${y}" font-family="Georgia, serif" font-size="18" font-weight="bold" fill="#1a1a1a">${escapeHtml(line.slice(3))}</text>`;
    } else if (line.match(/^[-*]\s/)) {
      return `<text x="${padding + 16}" y="${y}" font-family="JetBrains Mono, monospace" font-size="14" fill="#374151">${escapedLine}</text>`;
    } else if (line.match(/^\d+\.\s/)) {
      return `<text x="${padding}" y="${y}" font-family="JetBrains Mono, monospace" font-size="14" fill="#374151">${escapedLine}</text>`;
    }
    return `<text x="${padding}" y="${y}" font-family="JetBrains Mono, monospace" font-size="14" fill="#374151">${escapedLine}</text>`;
  }).join('\n');

  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${svgWidth} ${svgHeight}" width="${svgWidth}" height="${svgHeight}">
  <rect width="100%" height="100%" fill="#fefefe"/>
  <rect x="20" y="20" width="${svgWidth - 40}" height="${svgHeight - 40}" fill="none" stroke="#e5e7eb" stroke-width="1"/>
  <text x="${svgWidth / 2}" y="${padding + 30}" font-family="Georgia, serif" font-size="28" font-weight="bold" fill="#1a1a1a" text-anchor="middle">${escapeHtml(title)}</text>
  <line x1="${padding}" y1="${padding + titleHeight}" x2="${svgWidth - padding}" y2="${padding + titleHeight}" stroke="#e5e7eb" stroke-width="1"/>
  ${contentSvg}
</svg>`;
}

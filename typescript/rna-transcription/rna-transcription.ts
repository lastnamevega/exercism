export function toRna(dna: string) {
  if (!dna.match(/^[ACGT]+$/)) throw new Error('Invalid input DNA.');

  const mapping: Record<string, string> = { 'A': 'U', 'C': 'G', 'G': 'C', 'T': 'A' };

  return dna.split('').map(c => mapping[c]).join('');
}

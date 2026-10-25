export const conversionMap: Record<string, string[]> = {
  png: ["JPG", "WEBP", "BMP"],
  jpg: ["PNG", "WEBP", "BMP"],
  jpeg: ["PNG", "WEBP", "BMP"],
  webp: ["PNG", "JPG", "BMP"],
  bmp: ["PNG", "JPG", "WEBP"],

   docx: ["PDF"],
};

// const conversionMap: Record<string, string[]> = {
//   png: ["JPG", "WEBP", "BMP", "GIF"],
//   jpg: ["PNG", "WEBP", "BMP", "GIF"],
//   jpeg: ["PNG", "WEBP", "BMP", "GIF"],
//   webp: ["PNG", "JPG", "BMP", "GIF"],
//   bmp: ["PNG", "JPG", "WEBP", "GIF"],
//   gif: ["PNG", "JPG", "WEBP", "BMP"],

//   docx: ["PDF", "TXT", "HTML", "ODT"],
//   pdf: ["PNG", "JPG", "TXT"],

//   mp4: ["WEBM", "MOV", "GIF", "MP3"],
//   mp3: ["WAV", "OGG", "FLAC"],

//   csv: ["XLSX", "JSON", "TXT"],
//   xlsx: ["CSV", "PDF", "ODS"],
// };

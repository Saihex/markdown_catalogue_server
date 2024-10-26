export async function getFileInfo(
  filePath: string,
): Promise<[boolean, Deno.FileInfo | null]> {
  try {
    const fileInfo = await Deno.stat(filePath);
    return [true, fileInfo]; // File exists, return true and file info
  } catch (_err) {
    return [false, null]; // File does not exist, return false and null
  }
}

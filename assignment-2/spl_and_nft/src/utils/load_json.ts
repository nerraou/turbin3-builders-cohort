import { readFile } from "fs/promises";

export async function loadJson<T>(filename: string): Promise<T> {
  const content = await readFile(filename, { encoding: "utf-8" });

  return JSON.parse(content) as T;
}

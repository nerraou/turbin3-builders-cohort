import { writeFile } from "fs/promises";

export async function saveJson(filename: string, data: unknown) {
  await writeFile(filename, JSON.stringify(data, null, 2), {
    encoding: "utf-8",
  });
}

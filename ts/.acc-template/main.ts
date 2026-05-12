import { readFileSync } from "node:fs";

const input = readFileSync("/dev/stdin", "utf8").trim().split("\n");
const [n] = input[0].split(" ").map(Number);

console.log(n);

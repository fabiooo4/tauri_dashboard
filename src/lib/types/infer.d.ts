import { z } from "zod";
import { userSchema } from "./schemas";

declare global {
  type User = z.infer<typeof userSchema>;
}

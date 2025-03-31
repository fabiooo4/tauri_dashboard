import { z } from "zod";
import { userFormSchema } from "./schemas";
import { userSchema } from "./schemas";

declare global {
  type UserForm = z.infer<typeof userFormSchema>;
  type User = z.infer<typeof userSchema>;
}

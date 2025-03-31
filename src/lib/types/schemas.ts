import { z } from "zod";

export const userFormSchema = z.object({
  username: z.string(),
  password: z.string(),
});

export const userSchema = z.object({
  username: z.string(),
  password: z.string(),
  is_admin: z.boolean(),
});

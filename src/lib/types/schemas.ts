import { z } from "zod";

export const userSchema = z.optional(
  z.object({
    username: z.string(),
    password: z.string(),
  }),
);

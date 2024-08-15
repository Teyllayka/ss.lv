import { object, ObjectSchema, string, date, number, boolean } from "yup";

export let loginSchema = object({
  email: string().email().required(),
  password: string().required(),
});

export let registerSchema = object({
  email: string().email().required(),
  password: string().required(),
  name: string().required(),
  surname: string().required(),
  phone: string().required(),
  image: string().required(),
});

export async function validateSchema(schema: ObjectSchema<any>, fields: any) {
  const errors: { field: string; message: string }[] = [];
  try {
    await schema.validate(fields, { abortEarly: false });
  } catch (err: any) {
    err.inner.forEach((error: any) => {
      errors.push({
        field: error.path,
        message: error.message,
      });
    });
  }

  return errors;
}

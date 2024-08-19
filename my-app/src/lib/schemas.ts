import { object, ObjectSchema, string, number, mixed, ref } from "yup";

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
  repeatPassword: string().oneOf([ref('password')], 'Passwords must match').required(),
});

export let advertSchema = object({
  title: string().required(),
  description: string().required(),
  price: string().required(),
  image: string().required(),
  location: string().required(),
  condition: mixed().oneOf(["new", "used"]).required(),
});

export let advertCarSchema = object({
  engineFuelType: string().required(),
  engineVolume: number().required(),
  releaseYear: number().required(),
  mileage: number().required(),
  color: string().required(),
  brand: string().required(),
  model: string().required(),
});


export let advertElectronicsSchema = object({
  brand: string().required(),
  model: string().required(),
  color: string().required(),
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

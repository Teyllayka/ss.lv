import { object, ObjectSchema, string, number, mixed, ref } from "yup";

export let loginSchema = object({
  email: string().email().required(),
  password: string().required(),
});

interface RegisterFormValues {
  email: string;
  password: string;
  repeatPassword: string;
  companyName?: string;
  name?: string;
  surname?: string;
}

export const registerSchema: ObjectSchema<RegisterFormValues> = object({
  email: string()
    .email("Invalid email format")
    .required("Email is required"),
  
  password: string()
    .required("Password is required"),
  
  repeatPassword: string()
    .oneOf([ref('password')], 'Passwords must match')
    .required("Please confirm your password"),
  
  companyName: string()
    .trim(),
  
  name: string()
    .trim()
    .when('companyName', {
      is: (companyName: string | undefined) => !companyName || companyName.trim() === '',
      then: (schema) => schema.required('Name is required'),
      otherwise: (schema) => schema.notRequired(),
    }),
  
  surname: string()
    .trim()
    .when('companyName', {
      is: (companyName: string | undefined) => !companyName || companyName.trim() === '',
      then: (schema) => schema.required('Surname is required'),
      otherwise: (schema) => schema.notRequired(),
    }),
})
.test(
  'either-company-or-name-surname',
  'You must provide either a company name or both name and surname.',
  function (values) {
    const { companyName, name, surname } = values as RegisterFormValues;
    const hasCompany = companyName && companyName.trim() !== '';
    const hasName = name && name.trim() !== '';
    const hasSurname = surname && surname.trim() !== '';

    if (!hasCompany && (!hasName || !hasSurname)) {
      return this.createError({
        path: 'companyName',
        message: 'Company name is required',
      });
    }

    return true;
  }
);

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

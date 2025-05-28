import { object, ObjectSchema, string, number, mixed, ref, date, array } from "yup";

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
  email: string().email("Invalid email format").required("Email is required"),

  password: string()
    .required("Password is required")
    .min(8, "Password must be at least 8 characters long"),

  repeatPassword: string()
    .oneOf([ref("password")], "Passwords must match")
    .required("Please confirm your password"),

  companyName: string().trim(),

  name: string()
    .trim()
    .when("companyName", {
      is: (companyName: string | undefined) =>
        !companyName || companyName.trim() === "",
      then: (schema) => schema.required("Name is required"),
      otherwise: (schema) => schema.notRequired(),
    }),

  surname: string()
    .trim()
    .when("companyName", {
      is: (companyName: string | undefined) =>
        !companyName || companyName.trim() === "",
      then: (schema) => schema.required("Surname is required"),
      otherwise: (schema) => schema.notRequired(),
    }),
}).test(
  "either-company-or-name-surname",
  "You must provide either a company name or both name and surname.",
  function (values) {
    const { companyName, name, surname } = values as RegisterFormValues;
    const hasCompany = companyName && companyName.trim() !== "";
    const hasName = name && name.trim() !== "";
    const hasSurname = surname && surname.trim() !== "";

    if (!hasCompany && (!hasName || !hasSurname)) {
      return this.createError({
        path: "companyName",
        message: "Company name is required",
      });
    }

    return true;
  },
);

interface AdvertFormValues {
  title: string;
  description: string;
  price: string;
  location: string;
  photos: string[];
}

export const advertSchema: ObjectSchema<AdvertFormValues> = object({
  title: string().required("Title is required"),
  description: string()
    .required("Description is required")
    .min(100, "Description must be at least 100 characters long"),
  price: string().required("Price is required"),
  location: string().required("Location is required"),
   photos: array()
      .of(string().url("Invalid photo URL").required("Photos are required"))
      .min(1, "At least one photo is required")
      .required("Photos are required"),
});

export let contactSchema = object({
  name: string().required(),
  email: string().email().required(),
  message: string().required(),
  subject: string().required(),
});

export let resetPasswordSchema = object({
  password: string()
    .required("Password is required")
    .min(8, "Password must be at least 8 characters long"),

  confirmPassword: string()
    .oneOf([ref("password")], "Passwords must match")
    .required("Please confirm your password"),
});

export let forgotPasswordSchema = object({
  email: string().email("Invalid email format").required("Email is required"),
});

export let advertCarSchema = object({
  engineFuelType: string().required(),
  engineVolume: number().required(),
  enginePower: number().required(),
  fuelConsumption: number().required(),
  transmission: string().oneOf(["Manual","Automatic","Semi-Automatic"]).required(),
  bodyType: string().required(),
  releaseYear: number().required(),
  mileage: number().required(),
  seats: number().required(),
  doors: number().required(),
  color: string().required(),
  brand: string().required(),
  model: string().required(),
  VIN: string().length(17, "VIN must be 17 characters").required(),
  registrationDate: number().max(4).required(),
});

export let advertElectronicsSchema = object({
  brand: string().required(),
  modelNumber: string().required(),
  serialNumber: string().required(),
  warrantyPeriod: string().required(),
  releaseDate: number().max(4).required(),
  condition: string()
    .oneOf(["New","Like New","Used","For Parts"],"Invalid condition")
    .required(),
});

export let advertRealEstateSchema = object({
  propertyType: string().oneOf(["House","Apartment","Land","Commercial"]).required(),
  area: number().required(),
  bedrooms: number().required(),
  bathrooms: number().required(),
  floor: number().notRequired(),
  totalFloors: number().notRequired(),
  yearBuilt: number().required(),
  heatingType: string().required(),
});

export let advertFurnitureSchema = object({
  type: string().required(),
  material: string().required(),
  dimensions: string().required(),
  weight: number().notRequired(),
});

export let advertServiceSchema = object({
  serviceType: string().required(),
  availability: string().required(),
  hourlyRate: number().required(),
  experienceYears: number().required(),
});


export let editProfileSchema = object({
  name: string(),
  surname: string(),
  companyName: string(),
  phone: string().min(11),
  password: string().required("Password is required"),
});

export let editAdvertSchema = object({
  title: string().required("Title is required"),
  description: string()
    .required("Description is required")
    .min(100, "Description must be at least 100 characters long"),
  price: string().required("Price is required"),

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


export const fullAdvertSchemas: Record<string, ObjectSchema<any>> = {
  vehicles:    advertSchema.concat(advertCarSchema),
  electronics: advertSchema.concat(advertElectronicsSchema),
  "real-estate": advertSchema.concat(advertRealEstateSchema),
  furniture:   advertSchema.concat(advertFurnitureSchema),
  services:    advertSchema.concat(advertServiceSchema),
};

fullAdvertSchemas.default = advertSchema;

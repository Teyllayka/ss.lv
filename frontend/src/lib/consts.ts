import { env } from "$env/dynamic/public";



export const activeTabClass = "border-b-2 border-blue-500 text-blue-500";
export const inactiveTabClass =
  "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200";



export const socketChatUrl = env.PUBLIC_CHAT_API || "http://127.0.0.1:4000";
//export const chatUrl = !env.PUBLIC_CHAT_API?.includes('127.0.0.1') ? env.PUBLIC_CHAT_API + "/chat" : "http://127.0.0.1:4000";
export const chatUrl =  "http://127.0.0.1:4000";


export const categories = [
  { value: "electronics", label: "Electronics" },
  { value: "vehicles", label: "Vehicles" },
  { value: "furniture", label: "Furniture" },
  { value: "real-estate", label: "Real Estate" },
  { value: "services", label: "Services" },
];

export const categoryFields: Record<string, Array<any>> = {
  electronics: [
    { name: "brand", label: "Brand", type: "text" },
    { name: "modelNumber", label: "Model Number", type: "text" },
    { name: "serialNumber", label: "Serial Number", type: "text" },
    { name: "warrantyPeriod", label: "Warranty Period", type: "text" },
    { name: "releaseDate", label: "Release Date", type: "date" },
    {
      name: "condition",
      label: "Condition",
      type: "select",
      options: ["New", "Like New", "Used", "For Parts"],
    },
  ],
  vehicles: [
    { name: "engineFuelType", label: "Fuel Type", type: "select", options: ["Petrol", "Diesel", "Electric", "Hybrid"] },
    { name: "engineVolume", label: "Engine Volume (L)", type: "number" },
    { name: "enginePower", label: "Engine Power (HP)", type: "number" },
    { name: "fuelConsumption", label: "Fuel Consumption (L/100km)", type: "number" },
    { name: "transmission", label: "Transmission", type: "select", options: ["Manual", "Automatic", "Semi-Automatic"] },
    { name: "bodyType", label: "Body Type", type: "text" },
    { name: "releaseYear", label: "Release Year", type: "number" },
    { name: "mileage", label: "Mileage", type: "number" },
    { name: "seats", label: "Seats", type: "number" },
    { name: "doors", label: "Doors", type: "number" },
    { name: "color", label: "Color", type: "text" },
    { name: "brand", label: "Brand", type: "text" },
    { name: "model", label: "Model", type: "text" },
    { name: "VIN", label: "VIN", type: "text" },
    { name: "registrationDate", label: "Registration Date", type: "date" },
  ],
  furniture: [
    { name: "type", label: "Type", type: "text" },
    { name: "material", label: "Material", type: "text" },
    { name: "dimensions", label: "Dimensions", type: "text" },
    { name: "weight", label: "Weight", type: "number" },
  ],
  "real-estate": [
    { name: "propertyType", label: "Property Type", type: "select", options: ["House", "Apartment", "Land", "Commercial"] },
    { name: "area", label: "Area (sqm)", type: "number" },
    { name: "bedrooms", label: "Bedrooms", type: "number" },
    { name: "bathrooms", label: "Bathrooms", type: "number" },
    { name: "floor", label: "Floor", type: "number" },
    { name: "totalFloors", label: "Total Floors", type: "number" },
    { name: "yearBuilt", label: "Year Built", type: "number" },
    { name: "heatingType", label: "Heating Type", type: "text" },
  ],
  services: [
    { name: "serviceType", label: "Service Type", type: "text" },
    { name: "availability", label: "Availability", type: "text" },
    { name: "hourlyRate", label: "Hourly Rate", type: "number" },
    { name: "experienceYears", label: "Experience (years)", type: "number" },
  ],
};

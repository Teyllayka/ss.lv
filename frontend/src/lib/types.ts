type TabType = "profile" | "adverts" | "edit";
type ReviewTabType = "received" | "written";
type AdvertTabType = "active" | "sold";

interface UserData {
  name: string;
  surname: string;
  email: string;
  phone: string;
  emailVerified: boolean;
  telegramUsername: string;
  rating: number;
  adverts: any[];
  advertsWithReviews: any[];
  reviewedAdverts: any[];
  companyName: string;
  role: string;
}

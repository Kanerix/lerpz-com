const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001/api/v1";

class ApiService {
  private baseURL: string;

  constructor(baseURL: string) {
    this.baseURL = baseURL;
  }

  getUrl(endpoint: string): string {
    return `${this.baseURL}${endpoint}`;
  }
}

export const apiService = new ApiService(API_BASE_URL);

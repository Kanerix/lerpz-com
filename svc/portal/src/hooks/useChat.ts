import useSWR from "swr";
import { apiService } from "@/services/chat";
import { authenticatedFetch } from "@/utils/fetch";

export const apiKeys = {
  chat: () => apiService.getUrl("/chat"),
};

async function fetcher<T>(url: string): Promise<T> {
  const response = await authenticatedFetch(url);

  if (!response.ok) {
    const error = new Error("An error occurred while fetching the data.");
    (error as any).status = response.status;
    throw error;
  }

  return response.json();
}

export function useChat() {
  const { data, error, isLoading, mutate } = useSWR<any>(
    apiKeys.chat(),
    fetcher,
  );

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}

import { QueryClient } from "@tanstack/react-query"

let client: QueryClient | undefined

export const getQueryClient = (): QueryClient => {
	if (!client) {
		client = new QueryClient({
			defaultOptions: {
				queries: {
					staleTime: 30_000,
					retry: 1,
				},
			},
		})
	}
	return client
}

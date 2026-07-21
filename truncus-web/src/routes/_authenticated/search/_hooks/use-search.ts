import { useQuery } from "@tanstack/react-query"
import type { UseQueryResult } from "@tanstack/react-query"

import { api } from "#/libs/api/client.ts"
import type { TSearchHit } from "#/libs/api/types.ts"

type TParams = {
	q: string
	kind?: string
}

type TUseSearchReturn = {
	query: UseQueryResult<TSearchHit[], Error>
}

export const useSearch = ({ q, kind }: TParams): TUseSearchReturn => ({
	query: useQuery({
		queryKey: ["search", q, kind ?? ""],
		queryFn: async (): Promise<TSearchHit[]> => {
			const { hits } = await api.search({ q, kind, limit: 20 })
			return hits
		},
		enabled: q.trim().length > 0,
	}),
})

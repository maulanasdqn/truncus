import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import type { UseMutationResult, UseQueryResult } from "@tanstack/react-query"
import { useNavigate } from "@tanstack/react-router"
import { toast } from "sonner"

import { api } from "#/libs/api/client.ts"
import type { TDeleteResponse, TSessionMeta } from "#/libs/api/types.ts"

type TUseSessionReturn = {
	query: UseQueryResult<TSessionMeta, Error>
	deleteMutation: UseMutationResult<TDeleteResponse, Error, void>
}

export const useSession = (id: string): TUseSessionReturn => {
	const navigate = useNavigate()
	const queryClient = useQueryClient()
	const query = useQuery({
		queryKey: ["session", id],
		queryFn: () => api.getSession(id),
	})
	const deleteMutation = useMutation({
		mutationFn: () => api.deleteSession(id),
		onSuccess: async () => {
			toast.success("Session deleted")
			await queryClient.invalidateQueries({ queryKey: ["sessions"] })
			void queryClient.invalidateQueries({ queryKey: ["overview"] })
			await navigate({ to: "/sessions" })
		},
		onError: (error) => toast.error(error.message),
	})
	return { query, deleteMutation }
}

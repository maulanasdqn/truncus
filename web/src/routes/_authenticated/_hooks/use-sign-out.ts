import { useNavigate, useRouter } from "@tanstack/react-router"

import { clearToken } from "#/libs/auth/token.ts"
import { getQueryClient } from "#/libs/tanstack-query/index.ts"

type TUseSignOutReturn = {
	signOut: () => Promise<void>
}

export const useSignOut = (): TUseSignOutReturn => {
	const router = useRouter()
	const navigate = useNavigate()
	const signOut = async (): Promise<void> => {
		clearToken()
		getQueryClient().clear()
		await router.invalidate()
		await navigate({ to: "/login" })
	}
	return { signOut }
}

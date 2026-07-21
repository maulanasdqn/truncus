import { useNavigate, useRouter } from "@tanstack/react-router"
import { useState } from "react"

import { api } from "#/libs/api/client.ts"
import { setToken } from "#/libs/auth/token.ts"

type TUseLoginReturn = {
	token: string
	setTokenValue: (value: string) => void
	error: string | null
	isSubmitting: boolean
	submit: () => Promise<void>
}

export const useLogin = (): TUseLoginReturn => {
	const router = useRouter()
	const navigate = useNavigate()
	const [token, setTokenValue] = useState("")
	const [error, setError] = useState<string | null>(null)
	const [isSubmitting, setSubmitting] = useState(false)

	const submit = async (): Promise<void> => {
		const trimmed = token.trim()
		if (trimmed === "") return
		setSubmitting(true)
		setError(null)
		const ok = await api.verifyToken(trimmed).catch(() => false)
		if (!ok) {
			setError("That token was rejected. Check it and try again.")
			setSubmitting(false)
			return
		}
		setToken(trimmed)
		await router.invalidate()
		await navigate({ to: "/overview" })
	}

	return { token, setTokenValue, error, isSubmitting, submit }
}

import { createFileRoute } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"

import { TokenForm } from "./_components/token-form.tsx"

const LoginPage: FC = (): ReactElement => (
	<div className="flex min-h-screen items-center justify-center p-6">
		<TokenForm />
	</div>
)

export const Route = createFileRoute("/_public/login/")({
	component: LoginPage,
})

import { createFileRoute, Outlet, redirect } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"

const PublicLayout: FC = (): ReactElement => (
	<main className="min-h-screen bg-inverse">
		<Outlet />
	</main>
)

export const Route = createFileRoute("/_public")({
	beforeLoad: ({ context }) => {
		if (context.token) throw redirect({ to: "/overview" })
	},
	component: PublicLayout,
})

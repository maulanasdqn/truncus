import { createFileRoute, Outlet, redirect } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"

import { AppSidebar } from "./_components/app-sidebar.tsx"
import { MobileNav } from "./_components/mobile-nav.tsx"
import { useSignOut } from "./_hooks/use-sign-out.ts"

const AuthenticatedLayout: FC = (): ReactElement => {
	const { signOut } = useSignOut()
	return (
		<div className="flex min-h-screen flex-col bg-background lg:flex-row">
			<MobileNav onSignOut={signOut} />
			<AppSidebar onSignOut={signOut} />
			<main className="min-w-0 flex-1 p-4 pb-24 lg:p-8">
				<Outlet />
			</main>
		</div>
	)
}

export const Route = createFileRoute("/_authenticated")({
	beforeLoad: ({ context }) => {
		if (!context.token) throw redirect({ to: "/login" })
	},
	component: AuthenticatedLayout,
})

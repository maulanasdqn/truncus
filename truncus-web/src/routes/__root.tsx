import { QueryClientProvider } from "@tanstack/react-query"
import type { QueryClient } from "@tanstack/react-query"
import { createRootRouteWithContext, Outlet } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"

import { Toaster } from "#/components/ui/sonner.tsx"
import { getToken } from "#/libs/auth/token.ts"
import { getQueryClient } from "#/libs/tanstack-query/index.ts"

type TRouterContext = {
	queryClient: QueryClient
	token: string | null
}

const RootLayout: FC = (): ReactElement => (
	<QueryClientProvider client={getQueryClient()}>
		<Outlet />
		<Toaster position="top-right" />
	</QueryClientProvider>
)

export const Route = createRootRouteWithContext<TRouterContext>()({
	beforeLoad: () => ({ token: getToken() }),
	component: RootLayout,
})

import { createRouter } from "@tanstack/react-router"

import { getQueryClient } from "#/libs/tanstack-query/index.ts"
import { routeTree } from "./routeTree.gen.ts"

export const getRouter = () =>
	createRouter({
		routeTree,
		context: {
			queryClient: getQueryClient(),
			token: null,
		},
		scrollRestoration: true,
		defaultPreload: "intent",
		defaultPreloadStaleTime: 0,
	})

declare module "@tanstack/react-router" {
	interface Register {
		router: ReturnType<typeof getRouter>
	}
}

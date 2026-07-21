import tailwindcss from "@tailwindcss/vite"
import { tanstackRouter } from "@tanstack/router-plugin/vite"
import viteReact from "@vitejs/plugin-react"
import { defineConfig } from "vite"
import tsconfigPaths from "vite-tsconfig-paths"

const API_TARGET = process.env.TRUNCUS_API_URL ?? "https://truncus.stynx.app"

export default defineConfig({
	server: {
		proxy: {
			"/v1": {
				target: API_TARGET,
				changeOrigin: true,
			},
		},
	},
	plugins: [
		tsconfigPaths({ projects: ["./tsconfig.json"] }),
		tailwindcss(),
		tanstackRouter({
			target: "react",
			autoCodeSplitting: true,
			routesDirectory: "./src/routes",
			generatedRouteTree: "./src/routeTree.gen.ts",
			routeFileIgnorePattern:
				"^(_apis|_components|_data|_hooks|_constants|_utils)",
		}),
		viteReact(),
	],
})

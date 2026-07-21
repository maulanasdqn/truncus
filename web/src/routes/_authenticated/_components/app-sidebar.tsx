import { Link } from "@tanstack/react-router"
import { BrainCircuitIcon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { Button } from "#/components/ui/button.tsx"
import { BUILD_DATE, BUILD_REF } from "#/libs/build-info.ts"
import { NAV_ITEMS } from "../_constants/nav.ts"

type TProps = {
	onSignOut: () => void
}

export const AppSidebar: FC<TProps> = ({ onSignOut }): ReactElement => (
	<aside className="hidden w-60 shrink-0 flex-col justify-between bg-primary text-primary-foreground lg:flex">
		<div>
			<div className="flex items-center gap-3 px-6 py-8">
				<BrainCircuitIcon className="size-6" />
				<span className="text-sm font-semibold uppercase tracking-label">
					Truncus
				</span>
			</div>
			<nav aria-label="Primary" className="flex flex-col gap-1 px-3">
				{NAV_ITEMS.map((item) => (
					<Link
						key={item.to}
						to={item.to}
						className="flex items-center gap-3 px-3 py-2 text-xs font-light uppercase tracking-label text-secondary transition-colors hover:text-primary-foreground data-[status=active]:bg-primary-foreground data-[status=active]:font-medium data-[status=active]:text-primary"
					>
						<item.icon className="size-4" />
						{item.label}
					</Link>
				))}
			</nav>
		</div>
		<footer className="flex flex-col gap-4 border-t border-primary-foreground/20 px-6 py-6">
			<Button
				variant="outline"
				size="sm"
				className="border-primary-foreground/40 bg-transparent text-primary-foreground hover:bg-primary-foreground hover:text-primary"
				onClick={onSignOut}
			>
				Sign out
			</Button>
			<p className="text-[10px] font-light uppercase tracking-label text-secondary/70">
				Build {BUILD_REF}
				{BUILD_DATE ? ` · ${BUILD_DATE}` : ""}
			</p>
		</footer>
	</aside>
)

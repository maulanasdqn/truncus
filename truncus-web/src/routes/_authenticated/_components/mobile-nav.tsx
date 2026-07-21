import { Link } from "@tanstack/react-router"
import { BrainCircuitIcon, MenuIcon, XIcon } from "lucide-react"
import { useState } from "react"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { Button } from "#/components/ui/button.tsx"
import { NAV_ITEMS } from "../_constants/nav.ts"

type TProps = {
	onSignOut: () => void
}

export const MobileNav: FC<TProps> = ({ onSignOut }): ReactElement => {
	const [open, setOpen] = useState(false)
	return (
		<header className="sticky top-0 z-40 bg-primary text-primary-foreground lg:hidden">
			<div className="flex items-center justify-between px-4 py-3">
				<div className="flex items-center gap-2">
					<BrainCircuitIcon className="size-5" />
					<span className="text-sm font-semibold uppercase tracking-label">
						Truncus
					</span>
				</div>
				<Button
					variant="ghost"
					size="icon"
					className="text-primary-foreground hover:bg-primary-foreground/10"
					aria-label="Toggle menu"
					aria-expanded={open}
					onClick={() => setOpen((value) => !value)}
				>
					{match(open)
						.with(true, () => <XIcon />)
						.otherwise(() => <MenuIcon />)}
				</Button>
			</div>
			{match(open)
				.with(false, () => null)
				.otherwise(() => (
					<nav aria-label="Primary" className="flex flex-col gap-1 px-3 pb-3">
						{NAV_ITEMS.map((item) => (
							<Link
								key={item.to}
								to={item.to}
								onClick={() => setOpen(false)}
								className="flex items-center gap-3 px-3 py-2 text-xs font-light uppercase tracking-label text-secondary transition-colors hover:text-primary-foreground data-[status=active]:bg-primary-foreground data-[status=active]:font-medium data-[status=active]:text-primary"
							>
								<item.icon className="size-4" />
								{item.label}
							</Link>
						))}
						<Button
							variant="outline"
							size="sm"
							className="mt-2 border-primary-foreground/40 bg-transparent text-primary-foreground"
							onClick={() => {
								setOpen(false)
								onSignOut()
							}}
						>
							Sign out
						</Button>
					</nav>
				))}
		</header>
	)
}

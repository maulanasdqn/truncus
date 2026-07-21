import {
	BookOpenIcon,
	GraduationCapIcon,
	LayoutDashboardIcon,
	ListIcon,
	SearchIcon,
} from "lucide-react"
import type { LucideIcon } from "lucide-react"

export type TNavItem = {
	to: string
	label: string
	icon: LucideIcon
}

export const NAV_ITEMS: readonly TNavItem[] = [
	{ to: "/overview", label: "Overview", icon: LayoutDashboardIcon },
	{ to: "/sessions", label: "Sessions", icon: ListIcon },
	{ to: "/lessons", label: "Lessons", icon: GraduationCapIcon },
	{ to: "/knowledge", label: "Knowledge", icon: BookOpenIcon },
	{ to: "/search", label: "Search", icon: SearchIcon },
]

"use client"

import type * as React from "react"
import { Toaster as Sonner } from "sonner"

const Toaster = ({
	...props
}: React.ComponentProps<typeof Sonner>): React.ReactElement => {
	return (
		<Sonner
			className="toaster group"
			style={
				{
					"--normal-bg": "var(--popover)",
					"--normal-text": "var(--popover-foreground)",
					"--normal-border": "var(--border)",
				} as React.CSSProperties
			}
			{...props}
		/>
	)
}

export { Toaster }

import type { FC, ReactElement } from "react"
import ReactMarkdown from "react-markdown"
import type { Components } from "react-markdown"
import remarkGfm from "remark-gfm"

import { cn } from "#/lib/utils.ts"

const components: Components = {
	h1: ({ node, ...props }) => (
		<h1
			className="mt-5 mb-2 text-lg font-bold uppercase tracking-label first:mt-0"
			{...props}
		/>
	),
	h2: ({ node, ...props }) => (
		<h2
			className="mt-5 mb-2 text-base font-bold uppercase tracking-label first:mt-0"
			{...props}
		/>
	),
	h3: ({ node, ...props }) => (
		<h3
			className="mt-4 mb-1 text-sm font-semibold uppercase tracking-label first:mt-0"
			{...props}
		/>
	),
	p: ({ node, ...props }) => (
		<p className="my-2 text-sm leading-relaxed first:mt-0 last:mb-0" {...props} />
	),
	ul: ({ node, ...props }) => (
		<ul
			className="my-2 ml-5 list-disc space-y-1 text-sm leading-relaxed marker:text-muted-foreground"
			{...props}
		/>
	),
	ol: ({ node, ...props }) => (
		<ol
			className="my-2 ml-5 list-decimal space-y-1 text-sm leading-relaxed marker:text-muted-foreground"
			{...props}
		/>
	),
	li: ({ node, ...props }) => <li className="pl-1" {...props} />,
	a: ({ node, ...props }) => (
		<a
			className="font-medium text-primary underline underline-offset-2"
			target="_blank"
			rel="noreferrer"
			{...props}
		/>
	),
	strong: ({ node, ...props }) => <strong className="font-semibold" {...props} />,
	em: ({ node, ...props }) => <em className="italic" {...props} />,
	code: ({ node, className, ...props }) => (
		<code
			className={cn(
				"rounded-none bg-muted px-1 py-0.5 font-mono text-[0.85em]",
				className,
			)}
			{...props}
		/>
	),
	pre: ({ node, ...props }) => (
		<pre
			className="my-3 overflow-x-auto rounded-none bg-muted p-3 text-xs [&_code]:bg-transparent [&_code]:p-0"
			{...props}
		/>
	),
	blockquote: ({ node, ...props }) => (
		<blockquote
			className="my-3 border-l-2 border-border pl-3 text-sm text-muted-foreground"
			{...props}
		/>
	),
	hr: ({ node, ...props }) => <hr className="my-4 border-border" {...props} />,
	table: ({ node, ...props }) => (
		<div className="my-3 overflow-x-auto">
			<table className="w-full border-collapse text-sm" {...props} />
		</div>
	),
	th: ({ node, ...props }) => (
		<th
			className="border border-border px-2 py-1 text-left text-xs font-medium uppercase tracking-label"
			{...props}
		/>
	),
	td: ({ node, ...props }) => (
		<td className="border border-border px-2 py-1 align-top" {...props} />
	),
}

type TProps = {
	content: string
	className?: string
}

export const Markdown: FC<TProps> = ({ content, className }): ReactElement => (
	<div className={cn("text-foreground", className)}>
		<ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>
			{content}
		</ReactMarkdown>
	</div>
)

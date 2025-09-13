<script lang="ts">
	import { base } from '$app/paths';
</script>

<aside class="flex w-64 flex-col justify-between bg-[#181C21] p-6 print:hidden">
	<div>
		<div class="mb-8 flex items-center gap-2">
			<span class="material-symbols-outlined text-3xl text-[var(--primary-color)]"> insights </span>
			<h1 class="text-xl font-bold">File Analyzer</h1>
		</div>
		<nav class="flex flex-col gap-2">
			<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href={base || '/'}>
				<span class="material-symbols-outlined">dashboard</span>
				<span>Dashboard</span>
			</a>
			<a
				class="flex items-center gap-3 rounded-md bg-[var(--primary-color)] px-3 py-2 text-white"
				href={`${base}/info`}
			>
				<span class="material-symbols-outlined">description</span>
				<span>Files</span>
			</a>
			<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href="#">
				<span class="material-symbols-outlined">settings</span>
				<span>Settings</span>
			</a>
			<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href="#">
				<span class="material-symbols-outlined">help</span>
				<span>Help</span>
			</a>
			<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href="#">
				<span class="material-symbols-outlined">feedback</span>
				<span>Feedback</span>
			</a>
		</nav>
	</div>
	<div>
		<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href="#">
			<span class="material-symbols-outlined">logout</span>
			<span>Logout</span>
		</a>
	</div>
</aside>
<script lang="ts">
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';
	import {
		AlertDialog,
		AlertDialogContent,
		AlertDialogDescription,
		AlertDialogPortal,
		AlertDialogTitle,
		AlertDialogTrigger
	} from '@/shadcn-svelte-components/ui/alert-dialog';

	interface Props {
		branch: BranchInfo;
	}

	let { branch }: Props = $props();
</script>

<div class="w-full p-[6px]">
	<AlertDialog closeOnEscape closeOnOutsideClick>
		<AlertDialogTrigger
			class="w-full border rounded-[6px] hover:bg-accent hover:text-accent-foreground text-md"
		>
			Start Committing
		</AlertDialogTrigger>
		<AlertDialogPortal>
			<AlertDialogContent>
				<AlertDialogTitle>
					{branch.name}
				</AlertDialogTitle>
				<AlertDialogDescription>
					{branch.name}
				</AlertDialogDescription>
			</AlertDialogContent>
		</AlertDialogPortal>
	</AlertDialog>
</div>

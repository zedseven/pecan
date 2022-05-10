<script lang="ts" context="module">
	// View Mode
	export enum ViewMode {
		Edit = 0,
		View,
		Print,
	}
</script>

<script lang="ts">
	// Imports
	import { tick } from 'svelte';
	import { timeout } from '../util';
	import { printSettings } from '../stores';
	import editDeviceDetails from './editDeviceDetails.svelte';

	// Component Data
	export let deviceId = null;
	let viewMode = ViewMode.View;
	let printSettingsVisible = false;
	let isLoading = true;

	// Print the label
	const printLabel = async () => {
		// Set the view mode for printing (special layout)
		viewMode = ViewMode.Print;

		// Wait for the re-render to be complete
		await tick();
		while (isLoading) {
			await timeout(50);
		}

		// This blocks the whole tab until the user prints/doesn't print
		window.print();

		// Reset the view mode
		viewMode = ViewMode.View;
	};

	// Simply toggles the view mode on and off
	const toggleViewMode = () => {
		viewMode = viewMode ? ViewMode.Edit : ViewMode.View;
	};

	// Simply toggles the print settings on and off
	const togglePrintSettings = () => {
		printSettingsVisible = !printSettingsVisible;
	};

	$: slotIndex = $printSettings.slot - 1;
	$: rowsBeforeSlot = Math.floor(slotIndex / $printSettings.horizontalLabelCount);
	$: columnsBeforeSlot = slotIndex % $printSettings.horizontalLabelCount;
	$: rowHeight = '' + 99.0 / $printSettings.verticalLabelCount + 'vh'; // There's the tiniest overflow for some stupid reason when using 100vh as the base measurement
</script>

<div id="content">
	<div id="pageSettings" class="unprintable">
		<button id="viewModeToggle" on:click={toggleViewMode}>
			Switch to {viewMode ? 'Edit Mode' : 'View Mode'}
		</button>
		<button id="printSettingsToggle" on:click={togglePrintSettings}>
			{printSettingsVisible ? 'Hide' : 'Show'} Print Settings
		</button>
		{#if printSettingsVisible}
			<div id="printSettings">
				<table>
					<tr>
						<td>
							<label for="printSettingHorizontalLabelCount" class="block">Labels:</label>
						</td>
						<td>
							<label for="printSettingVerticalLabelCount" class="block">
								<input
									bind:value={$printSettings.horizontalLabelCount}
									id="printSettingHorizontalLabelCount"
									class="printSettingInput"
									type="number"
									min="1"
									max="4"
									placeholder="H"
								/>&nbsp;&times;&nbsp;<input
									bind:value={$printSettings.verticalLabelCount}
									id="printSettingVerticalLabelCount"
									class="printSettingInput"
									type="number"
									min="1"
									max="4"
									placeholder="V"
								/></label
							>
						</td>
					</tr>
					<tr>
						<td>
							<label for="printSettingLabelSlot" class="block">Slot:</label>
						</td>
						<td>
							<label for="printSettingLabelSlot" class="block">
								<input
									bind:value={$printSettings.slot}
									id="printSettingLabelSlot"
									class="printSettingInput"
									type="number"
									min="1"
									max={$printSettings.horizontalLabelCount * $printSettings.verticalLabelCount}
								/>
							</label>
						</td>
					</tr>
					<tr>
						<td>
							<label for="printSettingLabelMargin" class="block">Margin:</label>
						</td>
						<td>
							<label for="printSettingLabelMargin" class="block">
								<input
									bind:value={$printSettings.labelMargin}
									id="printSettingLabelMargin"
									class="printSettingInput"
									type="number"
									min="0"
								/>&nbsp;mm</label
							>
						</td>
					</tr>
					<tr>
						<td>
							<label for="printSettingFontSize" class="block">Font Size:</label>
						</td>
						<td>
							<label for="printSettingFontSize" class="block">
								<input
									bind:value={$printSettings.fontSize}
									id="printSettingFontSize"
									class="printSettingInput"
									type="number"
									min="1"
								/>&nbsp;pt</label
							>
						</td>
					</tr>
					<tr>
						<td>
							<label for="printSettingBorderMarkers" class="block">Show Boundary Guides:</label>
						</td>
						<td>
							<label for="printSettingBorderMarkers" class="block">
								<input
									bind:checked={$printSettings.borderMarkers}
									id="printSettingBorderMarkers"
									type="checkbox"
								/>
							</label>
						</td>
					</tr>
				</table>
				<button id="printButton" on:click={printLabel}>Print</button>
			</div>
		{/if}
		<br /><br />
	</div>
	{#if viewMode !== ViewMode.Print}
		<svelte:component this={editDeviceDetails} bind:deviceId bind:viewMode bind:isLoading />
	{:else}
		<table id="labelPrintTable" class:borderMarkers={$printSettings.borderMarkers}>
			<!-- Pad the area with empty rows before the display slot-->
			{#each Array(rowsBeforeSlot) as _}
				<tr class="noHoverDarken" style:height={rowHeight}>
					{#each Array($printSettings.horizontalLabelCount) as _}
						<td />
					{/each}
				</tr>
			{/each}
			<tr class="noHoverDarken">
				<!-- Pad the area with empty columns before the display slot -->
				{#each Array(columnsBeforeSlot) as _}
					<td />
				{/each}
				<td>
					<div
						id="label"
						style:height={rowHeight}
						style:padding={'' + $printSettings.labelMargin + 'mm'}
						style:font-size={'' + $printSettings.fontSize + 'pt'}
					>
						<div id="overflowContainer">
							<svelte:component
								this={editDeviceDetails}
								bind:deviceId
								bind:viewMode
								bind:isLoading
							/>
						</div>
					</div>
				</td>
				<!-- Pad the area with empty columns after the display slot -->
				{#each Array($printSettings.horizontalLabelCount - columnsBeforeSlot - 1) as _}
					<td />
				{/each}
			</tr>
			<!-- Pad the area with empty rows after the display slot-->
			{#each Array($printSettings.verticalLabelCount - rowsBeforeSlot - 1) as _}
				<tr class="noHoverDarken" style:height={rowHeight}>
					{#each Array($printSettings.horizontalLabelCount) as _}
						<td />
					{/each}
				</tr>
			{/each}
		</table>
	{/if}
</div>

<style lang="scss">
	#viewModeToggle {
		width: 12em;
	}
	#printSettingsToggle {
		width: 11em;
	}

	.printSettingInput {
		width: 3em;
	}
	#printButton {
		width: 6em;
	}

	#labelPrintTable {
		width: 100%;
		table-layout: fixed;
		border-collapse: collapse;
		page-break-inside: avoid;
	}
	#labelPrintTable tr,
	#labelPrintTable td,
	#label {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
		overflow: hidden;
	}
	#overflowContainer {
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
	:not(.borderMarkers) td {
		border: transparent dotted 0.5mm;
	}
	.borderMarkers td {
		border: gray dotted 0.5mm;
	}

	@media print {
		#labelPrintTable {
			height: 100vh;
		}
	}
</style>

<script lang="ts">
	// Imports
	import { emptyIfNull } from '../util';
	import datetime from './datetime.svelte';
	import { appName } from '../constants';

	// Component Data
	export let deviceId;
	export let changeList;
	export let locationsMap;
	export let columnDefinitionsMap;

	// Map for the display name of each operation
	let operationsMap = {
		add: 'Added',
		edit: 'Updated',
		delete: 'Deleted',
		restore: 'Restored',
	};
</script>

<div id="deviceChanges">
	<h3>History</h3>
	<table>
		{#each changeList as changeEntry}
			<tr>
				<td class="changeHeader">
					<svelte:component this={datetime} datetimeUtc={changeEntry.timestamp + 'Z'} />
				</td>
				<td class="changeHeader">
					{#if changeEntry.user != null}
						{changeEntry.user}
					{:else if changeEntry.doneAutomatically}
						<span
							class="italicised noSelect"
							title="This change was made automatically by the system."
						>
							&lt;{appName}&gt;
						</span>
					{:else}
						<span
							class="italicised noSelect"
							title="This change was likely made before change tracking was added to {appName}."
						>
							&lt;Unknown&gt;
						</span>
					{/if}
				</td>
				<td>
					<table class="subTable">
						{#if changeEntry.change.deviceKeyInfo != null}
							<tr>
								<th colspan="2">
									Device {operationsMap[changeEntry.change.deviceKeyInfo.operation]}
								</th>
							</tr>
						{/if}
						{#if changeEntry.change.deviceKeyInfo != null || changeEntry.change.deviceData != null}
							<tr><th>Device Info</th><th /></tr>
						{/if}
						{#if changeEntry.change.deviceKeyInfo != null}
							{#if changeEntry.change.deviceKeyInfo.locationId != null}
								<tr>
									<td class="dataHeader noSelect">Location:</td>
									<td>{emptyIfNull(locationsMap[changeEntry.change.deviceKeyInfo.locationId])}</td>
								</tr>
							{/if}
						{/if}
						{#if changeEntry.change.deviceData != null}
							{#each changeEntry.change.deviceData as dataChange}
								<tr>
									<td class="dataHeader noSelect">
										{emptyIfNull(columnDefinitionsMap[dataChange.columnDefinitionId][0].name)}:
									</td>
									<td>{dataChange.dataValue}</td>
								</tr>
							{/each}
						{/if}
						{#if changeEntry.change.deviceComponents != null && changeEntry.change.deviceComponents.length > 0}
							<tr><th>Components</th><th /></tr>
							{#each changeEntry.change.deviceComponents as componentChange}
								<tr>
									<td class="dataHeader">
										<span class="monospace">{deviceId}-{componentChange.componentId}</span><span
											class="noSelect"
											>:
											<span class="italicised">
												({operationsMap[componentChange.operation]})
											</span>
										</span>
									</td>
									<td>{emptyIfNull(componentChange.componentType)}</td>
								</tr>
							{/each}
						{/if}
						{#if changeEntry.change.deviceAttachments != null && changeEntry.change.deviceAttachments.length > 0}
							<tr><th>Attachments</th><th /></tr>
							{#each changeEntry.change.deviceAttachments as attachmentChange}
								<tr>
									<td class="dataHeader">
										<span class="monospace">{deviceId}-{attachmentChange.attachmentId}</span><span
											class="noSelect"
											>:
											<span class="italicised">
												({operationsMap[attachmentChange.operation]})
											</span>
										</span>
									</td>
									<td class="monospace">{emptyIfNull(attachmentChange.fileName)}</td>
								</tr>
								{#if attachmentChange.description != null && (attachmentChange.operation !== 'add' || attachmentChange.description !== '')}
									<tr>
										<td class="dataHeader noSelect">&emsp;Description:</td><td>
											{attachmentChange.description}
										</td>
									</tr>
								{/if}
							{/each}
						{/if}
					</table>
				</td>
			</tr>
		{/each}
	</table>
</div>

<style lang="scss">
	.changeHeader {
		vertical-align: top;
		padding: 4px 2px;
	}
	.subTable {
		width: 100%;
	}
	.dataHeader {
		width: 12em;
	}
</style>

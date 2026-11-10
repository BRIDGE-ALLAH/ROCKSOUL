-- One workspace inventory artifact per mission. This protects the local
-- review flow from duplicate scans even if commands race at the UI boundary.
CREATE UNIQUE INDEX IF NOT EXISTS idx_single_workspace_inventory_per_mission
ON artifacts (mission_id)
WHERE kind = 'workspace_inventory';

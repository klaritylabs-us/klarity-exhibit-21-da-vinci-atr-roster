
use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_roster_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Coverage") {
        r.push(ValidationIssue::error("resourceType", "ATR roster rows usually map to Coverage", "atr-coverage"));
    }
    if v.get("beneficiary").and_then(|b| b.get("reference")).is_none() {
        r.push(ValidationIssue::error("beneficiary", "beneficiary reference ties member to roster row", "beneficiary-ref"));
    }
    let period = v.get("period");
    if period.and_then(|p| p.get("start")).is_none() {
        r.push(ValidationIssue::error("period.start", "attribution window needs period.start", "period-start"));
    }
    if v.get("identifier").and_then(|i| i.as_array()).is_none_or(|a| a.is_empty()) {
        r.push(ValidationIssue::warning("identifier", "member id on Coverage helps reconcile roster files", "member-id"));
    }
    Ok(r)
}

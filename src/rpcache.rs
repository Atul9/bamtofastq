use std::collections::HashMap;
use rust_htslib::bam::record::{Aux, Record};

/// Read-pair cache. Let's us stream through the BAM and find nearby mates so we can write them out immediately
pub struct RpCache {
    pub cache: HashMap<Vec<u8>, Record>
}

impl RpCache {

    pub fn new() -> RpCache {
        RpCache { cache: HashMap::new() }
    }

    pub fn cache_rec(&mut self, rec: Record) -> Option<(Record, Record)> {
        // If cache already has entry, we have a pair! Return both
        match self.cache.remove(rec.qname()) {
            Some(old_rec) => {
                if rec.is_first_in_template() && old_rec.is_last_in_template() {
                    Some((rec, old_rec))
                } else if old_rec.is_first_in_template() && rec.is_last_in_template() {
                    Some((old_rec, rec))
                } else {
                    panic!("invalid pair")
                }
            },
            None => {
                self.cache.insert(Vec::from(rec.qname()), rec);
                None
            }
        }
    }

    pub fn clear_orphans(&mut self, current_tid: i32, current_pos: i32) -> Vec<Record> {
        let mut orphans = Vec::new();
        let mut new_cache = HashMap::new();

        for (key, rec) in self.cache.drain() {
            // Evict unmapped reads, reads on a previous chromosome, or reads that are >5kb behind the current position
            if rec.tid() == -1 || (current_pos - rec.pos()).abs() > 5000 || rec.tid() != current_tid {
                orphans.push(rec);
            } else {
                new_cache.insert(key, rec);
            }
        }

        self.cache = new_cache;
        orphans
    }

    pub fn len(&self) -> usize 
    {
        self.cache.len()
    }
}
# Provis

A live process monitor with graphical resource usage bars.

## Status

Work in progress - forked from [dysk](https://github.com/Canop/dysk) and evolving into a dedicated process visualization tool.

## Goal

Transform the disk monitoring architecture into a process monitoring system with visual graphs showing CPU, memory, I/O, and network usage per process.

## Platform

Linux-specific (uses /proc filesystem)

## Roadmap

- Replace disk data collection with /proc reading
- Add graphical bar rendering for resource usage
- Implement dynamic column management
- Add interactive sorting and filtering
- Support multiple view modes
- Color-coded thresholds
- Process grouping and historical trending

## License

MIT License - see LICENSE file

## Attribution

Thanks to Denys Seguret for the original dysk foundation.

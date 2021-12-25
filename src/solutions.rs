#[cfg(test)]
mod tests {
    use aoc_helpers::{Solution, Solver};
    use itertools::Itertools;

    use crate::{
        // alu::Computer,
        amphipod::Amphipod,
        bingo::{FastBoard, Runner},
        camera::Manual,
        cave::CaveSystem,
        chiton::ChitonGrid,
        crab::Crabs,
        cucumber::Cucumber,
        decoder::TransmissionWrapper,
        diagnostic::DiagnosticWrapper,
        dirac::Games,
        fish::{Homework, Sim},
        heightmap::HeightMap,
        navigation::Program,
        octopus::OctopusGrid,
        polymer::Polymerizer,
        probe::Launcher,
        reactor::Procedure,
        scanner::Mapper,
        sonar::Report,
        ssd::Matcher,
        submarine::Subs,
        trench::Enhancer,
        vents::Vents,
    };

    #[test]
    #[ignore]
    fn day_001() {
        let expected = Solution::new(1553, 1597);
        assert_eq!(Report::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_002() {
        let expected = Solution::new(1804520, 1971095320);
        assert_eq!(Subs::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_003() {
        let expected = Solution::new(4174964, 4474944);
        assert_eq!(DiagnosticWrapper::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_004() {
        let expected = Solution::new(49860, 24628);
        assert_eq!(Runner::<FastBoard>::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_005() {
        let expected = Solution::new(7414, 19676);
        assert_eq!(Vents::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_006() {
        let expected = Solution::new(379114, 1702631502303);
        assert_eq!(Sim::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_007() {
        let expected = Solution::new(349812, 99763899);
        assert_eq!(Crabs::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_008() {
        let expected = Solution::new(352, 936117);
        assert_eq!(Matcher::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_009() {
        let expected = Solution::new(436, 1317792);
        assert_eq!(HeightMap::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_010() {
        let expected = Solution::new(167379, 2776842859);
        assert_eq!(Program::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_011() {
        let expected = Solution::new(1546, 471);
        assert_eq!(OctopusGrid::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_012() {
        let expected = Solution::new(3679, 107395);
        assert_eq!(CaveSystem::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_013() {
        let expected = Solution::new(
            731,
            "
0000 0  0  00  0  0  00  0000 0  0  00
   0 0 0  0  0 0  0 0  0 0    0  0 0  0
  0  00   0  0 0  0 0    000  0  0 0
 0   0 0  0000 0  0 0    0    0  0 0
0    0 0  0  0 0  0 0  0 0    0  0 0  0
0000 0  0 0  0  00   00  0     00   00"
                .to_string(),
        );
        let actual = Manual::solve();
        assert_eq!(actual.part_one, expected.part_one);

        // my editor cleaves trailing whitespace, so we need to do that for the
        // actual solution
        let stripped = actual
            .part_two
            .lines()
            .map(|line| line.trim_end())
            .join("\n")
            .to_string();

        assert_eq!(stripped, expected.part_two);
    }

    #[test]
    #[ignore]
    fn day_014() {
        let expected = Solution::new(3259, 3459174981021);
        assert_eq!(Polymerizer::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_015() {
        let expected = Solution::new(447, 2825);
        assert_eq!(ChitonGrid::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_016() {
        let expected = Solution::new(955, 158135423448);
        assert_eq!(TransmissionWrapper::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_017() {
        let expected = Solution::new(12246, 3528);
        assert_eq!(Launcher::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_018() {
        let expected = Solution::new(4323, 4749);
        assert_eq!(Homework::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_019() {
        let expected = Solution::new(385, 10707);
        assert_eq!(Mapper::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_020() {
        let expected = Solution::new(4917, 16389);
        assert_eq!(Enhancer::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_021() {
        let expected = Solution::new(551901, 272847859601291);
        assert_eq!(Games::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_022() {
        let expected = Solution::new(545118, 1227298136842375);
        assert_eq!(Procedure::solve(), expected);
    }

    #[test]
    #[ignore]
    fn day_023() {
        let expected = Solution::new(14371, 40941);
        assert_eq!(Amphipod::solve(), expected);
    }

    // this is even too slow for the ignored tests
    // #[test]
    // #[ignore]
    // fn day_024() {
    //     let expected = Solution::new(29599469991739, 17153114691118);
    //     assert_eq!(Computer::solve(), expected);
    // }
    //
    #[test]
    #[ignore]
    fn day_025() {
        let expected = Solution::new(278, "No part 2 for day 25".to_string());
        assert_eq!(Cucumber::solve(), expected);
    }
}

/// Declares an application's own challenge types as one type that can be played, scored,
/// and saved like core's built-in challenges, and combined with them in a
/// [`ChallengeFamily`](crate::challenges::ChallengeFamily).
///
/// Each member is a type that implements `ChallengeDefinition`, `Playable`, `Clone`, `Debug`,
/// `PartialEq`, serde's `Serialize` and `Deserialize`, and whose `Result` and `Input` are
/// `Clone`, `Debug`, `PartialEq`, `Send`, `Sync`, and `'static`; the result is also
/// serializable. The macro generates three enums, one variant per member:
///
/// - the content enum, saved under each member's tag, with no marker of its own,
/// - the result enum, saved under the same tags,
/// - the input enum, which is never saved.
///
/// It also implements `ChallengeDefinition`, `Playable`, and `SerializedNames` by delegating
/// to the member, and `Includes<Member>` so the member's commands can be embedded.
///
/// Tags are the stable names used in saved games and assets. They must differ from each
/// other and from the names of any type this one is combined with
/// (see `ChallengeFamily::duplicate_names`). All members must use the declared `variant` type.
/// The crate using the macro needs a direct dependency on `serde`.
///
/// ```ignore
/// konnektoren_core::challenge_family! {
///     pub enum MusicChallenge, results MusicResult, inputs MusicInput, variant = ChallengeVariant {
///         Melody(Melody) = "melody",
///         Rhythm(Rhythm) = "rhythm",
///     }
/// }
/// ```
#[macro_export]
macro_rules! challenge_family {
    (
        $(#[$meta:meta])*
        $vis:vis enum $content:ident, results $results:ident, inputs $inputs:ident,
        variant = $variant:ty {
            $( $member:ident($ty:ty) = $tag:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        $vis enum $content {
            $( #[serde(rename = $tag)] $member($ty), )+
        }

        /// The results of the challenges above, saved under the same tags.
        #[derive(Clone, Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        $vis enum $results {
            $( #[serde(rename = $tag)] $member(<$ty as $crate::challenges::ChallengeDefinition>::Result), )+
        }

        /// The inputs of the challenges above. Inputs are never saved.
        #[derive(Clone, Debug, PartialEq)]
        $vis enum $inputs {
            $( $member(<$ty as $crate::challenges::Playable>::Input), )+
        }

        impl $crate::challenges::SerializedNames for $content {
            fn serialized_names() -> ::std::vec::Vec<&'static str> {
                ::std::vec![ $( $tag ),+ ]
            }
        }

        impl $crate::challenges::SerializedNames for $results {
            fn serialized_names() -> ::std::vec::Vec<&'static str> {
                ::std::vec![ $( $tag ),+ ]
            }
        }

        impl $crate::challenges::ChallengeDefinition for $content {
            type Result = $results;

            fn id(&self) -> &str {
                match self {
                    $( Self::$member(inner) => $crate::challenges::ChallengeDefinition::id(inner), )+
                }
            }

            fn of_tasks(&self, tasks: &$crate::challenges::task_pattern::TaskPattern) -> Self {
                match self {
                    $( Self::$member(inner) => Self::$member(
                        $crate::challenges::ChallengeDefinition::of_tasks(inner, tasks),
                    ), )+
                }
            }
        }

        impl $crate::challenges::Playable for $content {
            type Input = $inputs;
            type Variant = $variant;

            fn result_len(result: &$results) -> usize {
                match result {
                    $( $results::$member(inner) => <$ty as $crate::challenges::Playable>::result_len(inner), )+
                }
            }

            fn default_input(
                &self,
                task_index: usize,
            ) -> $crate::challenges::error::Result<::std::option::Option<$inputs>> {
                match self {
                    $( Self::$member(inner) => ::std::result::Result::Ok(
                        $crate::challenges::Playable::default_input(inner, task_index)?
                            .map($inputs::$member),
                    ), )+
                }
            }

            fn record_input(
                &self,
                result: &mut ::std::option::Option<$results>,
                input: $inputs,
            ) -> $crate::challenges::error::Result<()> {
                match (self, input) {
                    $( (Self::$member(inner), $inputs::$member(input)) => {
                        let mut stored = match result.take() {
                            ::std::option::Option::None => ::std::option::Option::None,
                            ::std::option::Option::Some($results::$member(stored)) => {
                                ::std::option::Option::Some(stored)
                            }
                            ::std::option::Option::Some(other) => {
                                *result = ::std::option::Option::Some(other);
                                return ::std::result::Result::Err(
                                    $crate::challenges::error::ChallengeError::FamilyMismatch,
                                );
                            }
                        };
                        let outcome = $crate::challenges::Playable::record_input(inner, &mut stored, input);
                        *result = stored.map($results::$member);
                        outcome
                    } )+
                    _ => ::std::result::Result::Err(
                        $crate::challenges::error::ChallengeError::FamilyMismatch,
                    ),
                }
            }

            fn try_performance(&self, result: &$results) -> $crate::challenges::error::Result<u32> {
                match (self, result) {
                    $( (Self::$member(inner), $results::$member(result)) => {
                        $crate::challenges::Playable::try_performance(inner, result)
                    } )+
                    _ => ::std::result::Result::Err(
                        $crate::challenges::error::ChallengeError::FamilyMismatch,
                    ),
                }
            }

            fn solve_task(
                challenge: &mut $crate::challenges::Attempt<Self>,
                input: $inputs,
                task_index: usize,
            ) -> $crate::challenges::error::Result<bool> {
                match (challenge.challenge_type.clone(), input) {
                    $( (Self::$member(inner), $inputs::$member(input)) => {
                        let stored = match challenge.challenge_result.take() {
                            ::std::option::Option::None => ::std::option::Option::None,
                            ::std::option::Option::Some($results::$member(stored)) => {
                                ::std::option::Option::Some(stored)
                            }
                            ::std::option::Option::Some(other) => {
                                challenge.challenge_result = ::std::option::Option::Some(other);
                                return ::std::result::Result::Err(
                                    $crate::challenges::error::ChallengeError::FamilyMismatch,
                                );
                            }
                        };
                        let mut attempt = $crate::challenges::Challenge {
                            challenge_type: inner,
                            challenge_config: challenge.challenge_config.clone(),
                            challenge_result: stored,
                            start_time: challenge.start_time,
                            end_time: challenge.end_time,
                        };
                        let outcome = <$ty as $crate::challenges::Playable>::solve_task(
                            &mut attempt,
                            input,
                            task_index,
                        );
                        challenge.challenge_result =
                            attempt.challenge_result.map($results::$member);
                        challenge.start_time = attempt.start_time;
                        challenge.end_time = attempt.end_time;
                        outcome
                    } )+
                    _ => ::std::result::Result::Err(
                        $crate::challenges::error::ChallengeError::FamilyMismatch,
                    ),
                }
            }

            fn option_input(&self, option_index: usize) -> $crate::challenges::error::Result<$inputs> {
                match self {
                    $( Self::$member(inner) => $crate::challenges::Playable::option_input(inner, option_index)
                        .map($inputs::$member), )+
                }
            }
        }

        $(
            impl $crate::challenges::Includes<$ty> for $content {
                fn embedding() -> $crate::challenges::ChallengeEmbedding<$ty, $content> {
                    $crate::challenges::ChallengeEmbedding::new(
                        $content::$member,
                        $results::$member,
                        $inputs::$member,
                        |variant| variant,
                    )
                    .with_extraction(
                        |content| match content {
                            $content::$member(inner) => ::std::option::Option::Some(inner.clone()),
                            #[allow(unreachable_patterns)]
                            _ => ::std::option::Option::None,
                        },
                        |result| match result {
                            $results::$member(inner) => ::std::option::Option::Some(inner.clone()),
                            #[allow(unreachable_patterns)]
                            _ => ::std::option::Option::None,
                        },
                        |variant| ::std::option::Option::Some(variant.clone()),
                    )
                }
            }
        )+
    };
}

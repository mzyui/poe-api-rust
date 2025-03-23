use serde::Serialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Serialize)]
pub enum QueryHash {
    #[default]
    SubscriptionsMutation,
    SendMessageMutation,
    HandleBotLandingPageQuery,
    HandleProfilePageQuery,
    MessageInfoPageQuery,
    messageSharing_shareMessagesMutation_Mutation,
    MessageDeleteConfirmationModal_deleteMessageMutation_Mutation,
    SettingsDeleteAllMessagesButton_deleteUserMessagesMutation_Mutation,
    SettingsDefaultBotSectionMutation,
    SettingsDefaultMessagePointLimitModal_SetAllChatDefaultMessagePointPriceThreshold_Mutation,
    ContinueChatCTAButton_continueChatFromPoeShare_Mutation,
    ChatSettingsModal_ChatSetTitle_Mutation,
    ChatSettingsModal_ChatSetContextOptimization_Mutation,
    useDeleteChat_deleteChat_Mutation,
    SearchResultsListPaginationQuery,
    ExploreBotsIndexPageQuery,
    ExploreBotsListPaginationQuery,
    ChatHistoryListPaginationQuery,
    UserFollowStateButton_poeUserSetFollow_Mutation,
    settingsPageQuery,
    ChatPageQuery,
    regenerateMessageMutation,
    cancelViewerActiveJobs_cancelViewerActiveJobs_Mutation,
    sendChatBreakMutation,
    useSharePreviewFromMessage_Mutation,
    CostThresholdUpdateChatModal_ChatSetMessagePointPriceThreshold_Mutation,
}

impl QueryHash {
    pub fn get_hash(&self) -> String {
        let hash = match *self {
            Self::CostThresholdUpdateChatModal_ChatSetMessagePointPriceThreshold_Mutation => {
                "8e36131a9013790c899523f76def20fe81da7cc69650b37ea076fd453b685682"
            }
            Self::useSharePreviewFromMessage_Mutation => {
                "56d6f245645427d368357d32dd444af37edaca497aa15219364864d0de495d41"
            }
            Self::sendChatBreakMutation => {
                "52035c9f0323132306b9cd36dd800edd3bc1418fc0d5cc1f6d1ed418155eaa8b"
            }
            Self::cancelViewerActiveJobs_cancelViewerActiveJobs_Mutation => {
                "bec4c5fb9ea395932da3174c38da893ffaf7ab142130ef1f0f796526051a80de"
            }
            Self::ChatPageQuery => {
                "82ea9ced7cb46a25ef787e118a00d27c6d69cec3791e0317c8f335b064d211a7"
            }
            Self::regenerateMessageMutation => {
                "0874efa8afdd12aedf1a4d14ccfd3a809d393d82c7dba12c23a0e3e0970ada09"
            }
            Self::settingsPageQuery => {
                "19f7f75aa4cc48a0a10c85f5be0190885659aeeb535507f6fa7e26485a069902"
            }
            Self::UserFollowStateButton_poeUserSetFollow_Mutation => {
                "8580c72320403ce5c3a00e88c2d52a8a54126a1b941af67662a1b5bfdce536ca"
            }
            Self::SubscriptionsMutation => {
                "5a7bfc9ce3b4e456cd05a537cfa27096f08417593b8d9b53f57587f3b7b63e99"
            }
            Self::SendMessageMutation => {
                "f1486efc974a214dac6586c46b81bf631a95e58eab1d27b215f622859d74a23e"
            }
            Self::HandleBotLandingPageQuery => {
                "2ec8856116b8c2cb587e9a05e60df21a751694b2ff06d67dfe0c3d0efaf5f6a2"
            }
            Self::HandleProfilePageQuery => {
                "ed55a4dd9ace8dfd13a7fa37ed009ac1b93c02f7e18eb42af81944ca76e8e45b"
            }
            Self::MessageInfoPageQuery => {
                "575cfeca537e4cc74e8ecc1fca0093e6ae988dc26b3cc66bbbb955b0880cde33"
            }
            Self::messageSharing_shareMessagesMutation_Mutation => {
                "652521d75d063d7665de9d96f690b61edcc24640f8428112b9490ebd307b1896"
            }
            Self::MessageDeleteConfirmationModal_deleteMessageMutation_Mutation => {
                "9f267eca67c714faa43fe19ec824ce0df5df504be8e7989f77bc748507cecaa5"
            }
            Self::SettingsDeleteAllMessagesButton_deleteUserMessagesMutation_Mutation => {
                "3f60d527c3f636f308b3a26fc3a0012be34ea1a201e47a774b4513d8a1ba8912"
            }
            Self::SettingsDefaultBotSectionMutation => {
                "4084604e8741af8650ac6b4236cdfa13c91a70cf1c63ad8a368706a386d0887e"
            }
            Self::SettingsDefaultMessagePointLimitModal_SetAllChatDefaultMessagePointPriceThreshold_Mutation => {
                "b3843325a4abf30891f1f99c1d87d9ca43761ce989be92fe93a986c55dedc4b6"
            }
            Self::ContinueChatCTAButton_continueChatFromPoeShare_Mutation => {
                "8b7bbb788463708e87ea979a383ddf6cbbb8818305add8b30c275a13ce9c7a95"
            }
            Self::ChatSettingsModal_ChatSetTitle_Mutation => {
                "3622c78363768bac6272765ffc507cd0416218917b3668ed39cced221de94c0f"
            }
            Self::ChatSettingsModal_ChatSetContextOptimization_Mutation => {
                "1e314e6829565b88fff37dfcbaab95dabe5338df7e231f7a2a2cb420545645b6"
            }
            Self::useDeleteChat_deleteChat_Mutation => {
                "5df4cb75c0c06e086b8949890b1871a9f8b9e431a930d5894d08ca86e9260a18"
            }
            Self::SearchResultsListPaginationQuery => {
                "a3db2f281540813c096123652d790d56c652fce0d3fca1ad234c81134d5de8f9"
            }
            Self::ExploreBotsIndexPageQuery => {
                "b8ca306feb56f998c46c23208109c4640d410616eb52f48444e5c54bac825438"
            }
            Self::ExploreBotsListPaginationQuery => {
                "b24b2f2f6da147b3345eec1a433ed17b6e1332df97dea47622868f41078a40cc"
            }
            Self::ChatHistoryListPaginationQuery => {
                "6ce01455b0201e625489da90c65f87a2809d212ea41ab6e39412b6913990e81f"
            }
        };
        hash.to_string()
    }
}

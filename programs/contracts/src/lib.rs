use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod DVideo {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn upload_video(ctx: Context<UploadVideo>, title: String, video_hash: String) -> ProgramResult {
        let dvideo_account = &mut ctx.accounts.dvideo_account;

        dvideo_account.videos.push(Video {
            title: title,
            hash: video_hash,
            publisher: ctx.accounts.publisher.key(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 6942)]
    pub dvideo_account: Account<'info, DVideoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UploadVideo<'info> {
    pub dvideo_account: Account<'info, DVideoAccount>,
    pub publisher: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Video {
    pub title: String,
    pub hash: String,
    pub publisher: Pubkey,
}

#[account]
pub struct DVideoAccount {
    pub name: String,
    pub videos: Vec<Video>,
}

#[error]
pub enum ErrorCode {
    #[msg("Title is empty")]
    NoTitle,
    #[msg("Video Hash is empty")]
    NoVideoHash,
}
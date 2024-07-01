use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("9D31LUnoNS7th12sKKZ1M8sYrhUbXaRK5RYQZ9D6tads");

#[program]
pub mod ecom {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let products = &mut ctx.accounts.products;
        let user = &mut ctx.accounts.user;

        products.total_products = 0;
        products.bump = ctx.bumps.products;
        products.admin =  *user.to_account_info().key;
        Ok(())    

    }

    pub fn add_product(
        ctx: Context<AddProduct>, 
        image_url: String, 
        price_in_sol: f64
    ) -> Result<()> {
        
        let products = &mut ctx.accounts.products;
        let user = &mut ctx.accounts.user;

        if user.key() != products.admin {
            return Err(ProgramError::IllegalOwner.into());
        }

        let item = ItemStruct {
            image_url: image_url.to_string(),
            price_in_sol: price_in_sol,
            owner_address: *user.to_account_info().key,
            listed: true
        };

        products.products_list.push(item);

        products.total_products = products.total_products.checked_add(1).unwrap();
        Ok(())

        // You do not need to change price_in_sol to anything since it's already in the correct type (f64). 
        // The image_url field required conversion to a String because it was initially passed as a &str. 
    }

    pub fn purchase_product(
        ctx: Context<PurchaseProduct>, 
        product_index: u64
    ) -> Result<()> {
        let products = &mut ctx.accounts.products;
        let buyer = &ctx.accounts.buyer;

        if product_index >= products.total_products {
            return Err(ProgramError::InvalidArgument.into());
        }

        let product = &mut products.products_list[product_index as usize];
        if !product.listed {
            return Err(ProgramError::InvalidArgument.into());
        }

        if product.owner_address != *ctx.accounts.to.key {
            return Err(ProgramError::IllegalOwner.into());
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
            });
        system_program::transfer(cpi_context, (product.price_in_sol * 1_000_000_000.0) as u64)?;


        product.listed = false; // Mark as sold
        product.owner_address = *buyer.key; // Update owner to buyer
        Ok(())
    }

    pub fn list_product(ctx: Context<ListProduct>, product_index: u64) -> Result<()> {
        let products = &mut ctx.accounts.products;
        let user = &ctx.accounts.user;
    
        if product_index >= products.total_products {
            return Err(ProgramError::InvalidArgument.into());
        }
    
        let product = &mut products.products_list[product_index as usize];
        if product.owner_address != *user.to_account_info().key {
            return Err(ProgramError::IllegalOwner.into());
        }
    
        product.listed = true;
        Ok(())
    }
    
    pub fn delist_product(ctx: Context<DelistProduct>, product_index: u64) -> Result<()> {
        let products = &mut ctx.accounts.products;
        let user = &ctx.accounts.user;
    
        if product_index >= products.total_products {
            return Err(ProgramError::InvalidArgument.into());
        }
    
        let product = &mut products.products_list[product_index as usize];
        if product.owner_address != *user.to_account_info().key {
            return Err(ProgramError::IllegalOwner.into());
        }
    
        product.listed = false;
        Ok(())
    }
    
    pub fn update_product_price(
        ctx: Context<UpdateProductPrice>, 
        product_index: u64, 
        new_price_in_sol: f64
    ) -> Result<()> {
        let products = &mut ctx.accounts.products;
        let user = &ctx.accounts.user;

        if product_index >= products.total_products {
            return Err(ProgramError::InvalidArgument.into());
        }

        let product = &mut products.products_list[product_index as usize];
        if product.owner_address != *user.to_account_info().key {
            return Err(ProgramError::IllegalOwner.into());
        }

        product.price_in_sol = new_price_in_sol;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"ecom4"], // optional seeds for pda
        bump,             // bump seed for pda
        payer = user,
        space = 10240 // max
    )]

    pub products: Account<'info, AllProducts>,

    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct AddProduct<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ecom4"], // optional seeds for pda
        bump = products.bump,  // bump seed for pda
    )]
    pub products: Account<'info, AllProducts>,

}


#[derive(Accounts)]
pub struct PurchaseProduct<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ecom4"],
        bump = products.bump,
    )]
    pub products: Account<'info, AllProducts>,

    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    /// CHECK: Account info of the product owner where ownership transfers
    pub to: AccountInfo<'info>, 
}


#[derive(Accounts)]
pub struct ListProduct<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ecom4"],
        bump = products.bump,
    )]
    pub products: Account<'info, AllProducts>,
}

#[derive(Accounts)]
pub struct DelistProduct<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ecom4"],
        bump = products.bump,
    )]
    pub products: Account<'info, AllProducts>,
}

#[derive(Accounts)]
pub struct UpdateProductPrice<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"ecom4"],
        bump = products.bump,
    )]
    pub products: Account<'info, AllProducts>,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub image_url: String,
    pub price_in_sol: f64,
    pub owner_address: Pubkey,
    pub listed: bool
}

#[account]
pub struct AllProducts {
    pub total_products: u64, // 8 bytes
    pub bump: u8,        // 1 byte
    pub products_list: Vec<ItemStruct>,
    pub admin: Pubkey
}


// Note:
// Reminder: when changing the structure of an account, you have to delete the 
// target and build again to deploy (it will give a new address)
// solana contracts are upgradable, but any change in account means change in contract

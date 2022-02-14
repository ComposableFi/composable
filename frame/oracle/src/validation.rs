
use composable_traits::math::SafeArithmetic;
use composable_support::validation::{Validate};
use frame_support::{pallet_prelude::*, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero};
use sp_runtime::Percent;
use crate::pallet::{Config};
use sp_std::fmt;

#[derive(Debug, Decode, Copy, Clone, PartialEq, TypeInfo)]
pub struct ValidMinAnswers;

#[derive(Debug, Copy, Clone, Decode, PartialEq, TypeInfo)]
pub struct ValidThreshhold;

#[derive(Debug, Decode)]
pub struct ValidMaxAnswer<U> {
   pub m: PhantomData<U>,
}

impl<U> Copy for ValidMaxAnswer<U> {}

impl<U> Clone for ValidMaxAnswer<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<MinAnswer: Zero + PartialEq + Eq + Ord +  PartialOrd> Validate<MinAnswer, ValidMinAnswers> for ValidMinAnswers {
    
    fn validate(input: MinAnswer) -> Result<MinAnswer, &'static str> {
      
       if input <= MinAnswer::zero() {
            return Err("INVALID_MIN_ANSWERS")
       }

       Ok(input)
    }
}

impl Validate<Percent,ValidThreshhold > for ValidThreshhold {
    fn validate(input: Percent) -> Result<Percent, &'static str> {
        
        if input >= Percent::from_percent(100) {
          return Err("INVALID_THRESHOLD");
        }  

        Ok(input)
    }
}

impl<MaxAnswer: PartialEq + PartialOrd, MaxAnswerBound> 
    Validate<MaxAnswer, ValidMaxAnswer<MaxAnswerBound>>
       for ValidMaxAnswer<MaxAnswerBound>  where  MaxAnswerBound: Get<MaxAnswer> 
       {
        fn validate(input: MaxAnswer) -> Result<MaxAnswer, &'static str>{
            
            if input > MaxAnswerBound::get() {
                return Err("INVALID_MAX_ANSWER")
            }

            Ok(input)
        }
 }
    
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;
;;  LvlUp_AdjustBBSubStats  [$9966 :: 0x2D976]
;;
;;  Adjusts post level up substats for Black Belts / Masters
;;
;;  input:  lvlup_chstats should be prepped
;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

LvlUp_AdjustBBSubStats:
    LDY #ch_class - ch_stats        ; check to make sure this is a BB/Master
    LDA (lvlup_chstats), Y
    CMP #CLS_BB
    BEQ :+
    CMP #CLS_MA
    BEQ :+                          ; if yes, jump ahead, otherwise just exit
  @Exit:
    RTS

  : LDY #ch_weapons - ch_stats      ; see if they have any weapon equipped.
    LDA (lvlup_chstats), Y          ; check all 4 weapon slots, if any of them have an
    BMI @Exit                       ; equipped weapon, exit
    INY
    LDA (lvlup_chstats), Y
    BMI @Exit
    INY
    LDA (lvlup_chstats), Y
    BMI @Exit
    INY
    LDA (lvlup_chstats), Y
    BMI @Exit
    
    LDY #ch_level - ch_stats        ; reaches here if no weapon equipped.  Get the level
    LDA (lvlup_chstats), Y          ;  Add 1 to make it 1-based
    CLC
    ADC #$01
    LDY #ch_absorb - ch_stats       ; Absorb = Level -- BUGGED:  This is the infamous BB Armor Bug
    STA (lvlup_chstats), Y          ;   This should only happen if the character has no ARMOR equipped.
                                    ;   Weapons shouldn't matter.  This cannot be easily fixed here,
                                    ;   as you'd pretty much have to write a new routine.
    ASL A
    LDY #ch_dmg - ch_stats          ; Damage = 2*Level
    STA (lvlup_chstats), Y
    RTS

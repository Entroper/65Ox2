;; variables.asm
unsram          = $6000  ; $400 bytes

ch_stats        = unsram + $0100  ; MUST be on page bound.  Each character allowed $40 bytes, so use 00,40,80,C0 to index ch_stats
ch_class        = ch_stats + $00
ch_weapons      = ch_stats + $18  ; 4
ch_level        = ch_stats + $26        ; OB this is 0 based, IB this is 1 based

ch_substats     = ch_stats + $20
ch_dmg          = ch_substats + $00
ch_absorb       = ch_substats + $02

lvlup_chstats       = $86       ; 2 byte pointer to character's OB stats

CLS_BB  = $02
CLS_MA  = $08

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
    ASL
    LDY #ch_dmg - ch_stats          ; Damage = 2*Level
    STA (lvlup_chstats), Y
    RTS

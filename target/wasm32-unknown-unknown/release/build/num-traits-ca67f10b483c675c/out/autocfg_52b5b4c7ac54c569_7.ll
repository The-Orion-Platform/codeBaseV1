; ModuleID = 'autocfg_52b5b4c7ac54c569_7.dccccfea6c72060b-cgu.0'
source_filename = "autocfg_52b5b4c7ac54c569_7.dccccfea6c72060b-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h77fdb9a8b870fc0fE"(ptr sret([8 x i8]) align 1 %_0, double %self) unnamed_addr #0 {
start:
  %self1 = bitcast double %self to i64
  store i64 %self1, ptr %_0, align 1
  ret void
}

; autocfg_52b5b4c7ac54c569_7::probe
; Function Attrs: nounwind
define dso_local void @_ZN26autocfg_52b5b4c7ac54c569_75probe17hc7aec0c2036d15dfE() unnamed_addr #1 {
start:
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  call void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h77fdb9a8b870fc0fE"(ptr sret([8 x i8]) align 1 %_1, double 3.140000e+00) #2
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.83.0 (90b35a623 2024-11-26)"}

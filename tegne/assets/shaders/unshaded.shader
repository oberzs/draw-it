vert.spv                                                                                                                    00000010364 00000000000 0005345                                                                                                      ustar                                                                                                                                                                                                                                                          #     b                 GLSL.std.450                      main       3   6   :   <   >   K   P   R   X   [   ]        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     $   WorldObject   $       world_matrix      $      light_matrix      $      lights    $      camera_position   $      time      $      cascade_splits    $      shadow_index      &   world     ,   lightspace_position   3   out_modelspace_position   6   out_worldspace_position  	 :   out_screenspace_position      <   out_lightspace_position   >   out_normal    K   in_normal     P   out_uv    R   in_uv     V   gl_PerVertex      V       gl_Position   V      gl_PointSize      V      gl_ClipDistance   V      gl_CullDistance   X         [   out_color     ]   in_color      _   MaterialObject    _       albedo_tint   _      font_width    _      font_border_tint      _      font_edge     _      font_border_offset    _      font_border_width     _      font_border_edge      _      arg_1     _      arg_2     _   	   arg_3     _   
   arg_4     a   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          H  $          H  $       #       H  $             H  $         H  $      #   @   H  $            H  $      #   �   H  $      #      H  $      #     H  $      #     H  $      #     G  $      G  &   "       G  &   !       G  3         G  6         G  :         G  <         G  >          G  K         G  P         G  R         H  V              H  V            H  V            H  V            G  V      G  [         G  ]         H  _       #       H  _      #      H  _      #      H  _      #      H  _      #       H  _      #   (   H  _      #   ,   H  _      #   0   H  _      #   @   H  _   	   #   P   H  _   
   #   `   G  _      G  a   "      G  a   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "    	 $         #   
      
         %      $   ;  %   &         '         +     -         2      
   ;  2   3      ;  2   6         9         ;  9   :      ;  9   <      ;  2   >        C   
      ;     K        N            O      N   ;  O   P         Q      N   ;  Q   R      +  !   T        U      T     V         U   U      W      V   ;  W   X      ;  9   [         \         ;  \   ]        _   
      
      N                        `      _   ;  `   a      6               �     ;     	      ;           ;           ;     ,      =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  '   (   &      =     )   (   =     *      �     +   )   *   >     +   A  '   .   &   -   =     /   .   =     0      �     1   /   0   >  ,   1   =     4   	   O  
   5   4   4             >  3   5   =     7      O  
   8   7   7             >  6   8   =     ;      >  :   ;   =     =   ,   >  <   =   A     ?         =     @   ?        A      "   @   T     B   A   Q     D   B       O  
   E   D   D             Q     F   B      O  
   G   F   F             Q     H   B      O  
   I   H   H             P  C   J   E   G   I   =  
   L   K   �  
   M   J   L   >  >   M   =  N   S   R   >  P   S   =     Y      A  9   Z   X      >  Z   Y   =     ^   ]   >  [   ^   �  8                                                                                                                                                                                                                                                                              frag.spv                                                                                                                    00000006524 00000000000 0005307                                                                                                      ustar                                                                                                                                                                                                                                                          #     M                 GLSL.std.450                     main       -   F   H   I   J   K   L                �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         fragment(        out_color        textures         Constants            model_matrix            albedo_index            sampler_index        object    "   samplers      -   in_uv     1   MaterialObject    1       albedo_tint   1      font_width    1      font_border_tint      1      font_edge     1      font_border_offset    1      font_border_width     1      font_border_edge      1      arg_1     1      arg_2     1   	   arg_3     1   
   arg_4     3   material      ?   Light     ?       coords    ?      color     B   WorldObject   B       world_matrix      B      light_matrix      B      lights    B      camera_position   B      time      B      cascade_splits    B      shadow_index      D   world     F   in_normal     H   in_color      I   in_modelspace_position    J   in_worldspace_position    K   in_screenspace_position   L   in_lightspace_position  G            G     "      G     !       H            H         #       H               H        #   @   H        #   D   G        G  "   "      G  "   !      G  -         H  1       #       H  1      #      H  1      #      H  1      #      H  1      #       H  1      #   (   H  1      #   ,   H  1      #   0   H  1      #   @   H  1   	   #   P   H  1   
   #   `   G  1      G  3   "      G  3   !       H  ?       #       H  ?      #      G  A          H  B          H  B       #       H  B             H  B         H  B      #   @   H  B            H  B      #   �   H  B      #      H  B      #     H  B      #     H  B      #     G  B      G  D   "       G  D   !       G  F          G  H         G  I         G  J         G  K         G  L              !                   	            
      	   ;  
          	                                          +        d                           ;                 	                                      	      ;        	   +                 	                        +                          !           ;  !   "       +     #         &            )        +            ,      +   ;  ,   -        0           1   0      0      +         	   	   	   	      2      1   ;  2   3      +     4          5      0   +     8     �?  ?   	   	   +     @        A   ?   @    	 B         A   0      0         C      B   ;  C   D         E      0   ;  E   F         G      	   ;  G   H      ;  E   I      ;  E   J      ;  G   K      ;  G   L      6               �     9     >      �  8  6               �     A              =           A              =           A     $      #   =     %   $   A  &   '   "   %   =     (   '   V  )   *      (   =  +   .   -   W  	   /   *   .   A  5   6   3   4   =  0   7   6   Q     9   7       Q     :   7      Q     ;   7      P  	   <   9   :   ;   8   �  	   =   /   <   >     =   �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
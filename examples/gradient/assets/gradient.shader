vert.spv                                                                                                                    00000010764 00000000000 0005351                                                                                                      ustar                                                                                                                                                                                                                                                          #     p                 GLSL.std.450                      main       .   1   5   9   L   Y   ^   `   f   i   k        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      bias      '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   L   out_normal    Y   in_normal     ^   out_uv    `   in_uv     d   gl_PerVertex      d       gl_Position   d      gl_PointSize      d      gl_ClipDistance   d      gl_CullDistance   f         i   out_color     k   in_color      m   MaterialObject    m       albedo_tint   m      font_width    m      font_border_tint      m      font_edge     m      font_border_offset    m      font_border_width     m      font_border_edge      m      arg_1     m      arg_2     m   	   arg_3     m   
   arg_4     o   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  L          G  Y         G  ^         G  `         H  d              H  d            H  d            H  d            G  d      G  i         G  k         H  m       #       H  m      #      H  m      #      H  m      #      H  m      #       H  m      #   (   H  m      #   ,   H  m      #   0   H  m      #   @   H  m   	   #   P   H  m   
   #   `   G  m      G  o   "      G  o   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    	 %      #   
      $            &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      ;  -   L        Q   
      ;     Y        \            ]      \   ;  ]   ^         _      \   ;  _   `      +  !   b        c      b     d         c   c      e      d   ;  e   f      ;  4   i         j         ;  j   k        m   
      
      \                        n      m   ;  n   o      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A     M         =     N   M        O      "   N   T     P   O   Q     R   P       O  
   S   R   R             Q     T   P      O  
   U   T   T             Q     V   P      O  
   W   V   V             P  Q   X   S   U   W   =  
   Z   Y   �  
   [   X   Z   >  L   [   =  \   a   `   >  ^   a   =     g   	   A  4   h   f      >  h   g   =     l   k   >  i   l   �  8              frag.spv                                                                                                                    00000007244 00000000000 0005307                                                                                                      ustar                                                                                                                                                                                                                                                          #     U                 GLSL.std.450                     main       (   L   N   O   P   Q   T                �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         fragment(     
   percent      in_uv        color        MaterialObject           albedo_tint         font_width          font_border_tint            font_edge           font_border_offset          font_border_width           font_border_edge            arg_1           arg_2        	   arg_3        
   arg_4        material      (   out_color     1   Light     1       coords    1      color     5   WorldObject   5       world_matrix      5      lights    5      camera_position   5      time      5      light_matrices    5      cascade_splits    5      bias      7   world     8   Constants     8       model_matrix      8      albedo_index      8      sampler_index     :   object    ?   textures      D   samplers      H   shadow_maps   J   framebuffer   L   in_normal     N   in_color      O   in_modelspace_position    P   in_worldspace_position    Q   in_screenspace_position   T   in_lightspace_position  G           H         #       H        #      H        #      H        #      H        #       H        #   (   H        #   ,   H        #   0   H        #   @   H     	   #   P   H     
   #   `   G        G     "      G     !       G  (          H  1       #       H  1      #      G  3          G  4      @   H  5          H  5       #       H  5             H  5      #   @   H  5      #   �   H  5      #   �   H  5         H  5      #   �   H  5            H  5      #   �  H  5      #   �  G  5      G  7   "       G  7   !       H  8          H  8       #       H  8             H  8      #   @   H  8      #   D   G  8      G  ?   "      G  ?   !       G  D   "      G  D   !      G  H   "      G  H   !       G  J   "      G  J   !       G  L          G  N         G  O         G  P         G  Q         G  T              !                    	                                ;                        +                                                                                                            ;                       +                       +               '         ;  '   (      +     *     �?  0           1         +     2        3   1   2     4   0   2    	 5   0   3         4            6      5   ;  6   7        8   0            9   	   8   ;  9   :   	    	 ;                            +     <   d     =   ;   <      >       =   ;  >   ?         @   +     A        B   @   A      C       B   ;  C   D       +     E        F   ;   E      G       F   ;  G   H          I       ;   ;  I   J          K         ;  K   L         M         ;  M   N      ;  K   O      ;  K   P      ;  M   Q        R      2      S      R   ;  S   T      6               �     9     /      �  8  6               �     ;  	   
      ;           A              =           >  
      A              =           O                        A     !          =     "   !   O     #   "   "             =     $   
   P     %   $   $   $        &      .      #   %   >     &   =     )      Q     +   )       Q     ,   )      Q     -   )      P     .   +   ,   -   *   >  (   .   �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              